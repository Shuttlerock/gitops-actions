use git2::{ Commit, Cred, CredentialType, FetchOptions, ObjectType, PushOptions, Repository, RemoteCallbacks, Signature };
use git2::build::RepoBuilder;
use std::path::{ Path, PathBuf };
use std::fs;
use std::io;

pub struct Context {
    user: String,
    email: String,
    password: Option<String>,

    repository: Repository,
    directory: PathBuf,
}

impl Context {
    pub fn checkout(&self, branch: &str) -> Result<(), git2::Error> {
        let obj = self.repository.revparse_single(&format!("refs/heads/{}", branch))?;
        self.repository.checkout_tree(&obj, None)?;

        Ok(())
    }

    fn find_last_commit(&self) -> Result<Commit, git2::Error> {
        let obj = self.repository.head()?.resolve()?.peel(ObjectType::Commit)?;
        obj.into_commit().map_err(|_| git2::Error::from_str("Couldn't find commit"))
    }

    pub fn add_and_commit(&self, path: &Path, message: &str) -> Result<(), git2::Error> {
        let mut index = self.repository.index()?;
        index.add_path(path)?;
        let oid = index.write_tree()?;
        let signature = Signature::now(&self.user, &self.email)?;
        let parent_commit = self.find_last_commit()?;
        let tree = self.repository.find_tree(oid)?;

        self.repository.commit(
            Some("HEAD"),
            &signature,
            &signature,
            message,
            &tree,
            &[&parent_commit])?;

        Ok(())
    }

    pub fn push_head(&self, remote_branch: &str) -> Result<(), git2::Error> {
        let mut remote = self.repository.find_remote("origin")?;

        let mut opts = PushOptions::new();

        let callbacks = remote_callbacks(&self.user, self.password.as_deref());
        opts.remote_callbacks(callbacks);

        remote.push(&[format!("HEAD:refs/heads/{}", remote_branch)], Some(&mut opts))?;

        Ok(())
    }

    pub fn cleanup(&self) -> Result<(), io::Error> {
        fs::remove_dir_all(&self.directory)?;
        Ok(())
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        self.cleanup().ok();
    }
}

fn remote_callbacks<'a>(user: &'a str, password: Option<&'a str>) -> RemoteCallbacks<'a> {
    let mut callbacks = RemoteCallbacks::new();

    callbacks.credentials(move |_user, _user_from_url, cred| {
        if cred != CredentialType::USER_PASS_PLAINTEXT {
            return Err(git2::Error::from_str("unsupported authentication requested"));
        }

        if let Some(password) = password {
            return Cred::userpass_plaintext(user, password);
        }

        Err(git2::Error::from_str("repository requires password, but none supplied"))
    });

    callbacks
}

pub fn clone(url: &str, path: &Path, user: &str, email: &str, password: Option<&str>) -> Result<Context, git2::Error> {
    let mut opts = FetchOptions::new();

    let callbacks = remote_callbacks(user, password);
    opts.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(opts);

    println!("Cloning repository {}...", url);

    let repository = builder.clone(url, path.clone())?;

    println!("Cloning complete.");

    let ctx = Context {
        directory: path.to_path_buf(),
        repository,
        user: user.to_string(),
        email: email.to_string(),
        password: password.map(String::from),
    };

    Ok(ctx)
}
