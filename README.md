# git-ll: Lazy Llama

Simple goal is to improve DX (Developer Experience) and make using git more productive on terminal.

`git-ll` is a command-line tool that integrates Git with the power of the `LLaMA 3.1` language model, providing intelligent assistance for your Git workflow. By leveraging the offline capabilities of the Ollama CLI and Meta's llama3.1 language model, this tool offers a secure and efficient way to generate commit messages, analyze code changes, and receive insightful suggestions directly from your local environment and terminal.

# Features
- [x] git running with llama3.1 , power of AI right in your terminal.
- [x] Comprehensive Diff analysis with AI-driven explanations and insights.
- [x] Craft concise and informative commit messages based on your code changes.
- [x] Ask your doubts for git, right in your terminal
- [x] Automate deleting a group of branches.
- [x] Easily stage the files from local to staging.
- [x] Checkout faster between multiple branches.

# Installation

Note : rust crate and homebrew coming soon but for now, I am just supporting local installation via git.

Make sure you already have git configured in your terminal and ollama too (only if you use AI powered features)
```bash
git clone git@github.com:Harshil-Jani/git-ll.git
cd git-ll/
cargo build --release
cargo install --path .
```

Must : Ollama CLI installed and configured with LLaMA 3.1 model.
Must : git CLI should be configured.

# Available commands:

- ```git-ll ask "How to merge two branches in git?"```
- ```git-ll diff <commit-1 : optional> <commit-2: optional>```
- ```git-ll add .``` OR ```git-ll add all``` OR ```git-ll add```
- ```git-ll commit```
- ```git-ll checkout``` OR ```git-ll checkout local``` OR ```git-ll checkout remote```
- ```git-ll delete```

# Flexing my tool

### Asking a doubt to llama3.1

<img width="1727" alt="Screenshot 2024-08-15 at 5 40 11 PM" src="https://github.com/user-attachments/assets/f480422c-99b8-48af-80e2-2a9c5bc89d4e">

### Automatically generating a commit message

<img width="1726" alt="Screenshot 2024-08-16 at 1 21 18 AM" src="https://github.com/user-attachments/assets/e509a039-3a10-4a65-81d9-aba7949a560a">

### Adding the file to staging with checkboxes that makes it easier to use

<img width="1726" alt="Screenshot 2024-08-16 at 1 20 30 AM" src="https://github.com/user-attachments/assets/f5eb3f0d-ff72-4b88-b50a-c7a6e52a3335">

### Analysing the difference between the two commited changes (or you can just ask difference between current change and HEAD)

<img width="1726" alt="Screenshot 2024-08-16 at 1 16 18 AM" src="https://github.com/user-attachments/assets/d6b6df2a-7cd7-427e-8fcd-30577ebedd68">

### Checkout to different branches quickly without having to worry about typing or copying full name

<img width="1726" alt="Screenshot 2024-08-16 at 1 18 31 AM" src="https://github.com/user-attachments/assets/9a84cc99-48f8-40c1-9dc2-c1f7e8a9f097">

### Delete more than one branches at a time without even typing the name

https://github.com/user-attachments/assets/7405d9d8-30db-4305-82c3-46c7d7e804fb

# Contributing
We welcome contributions to git-ll! Please feel free to open issues or submit pull requests.

# License
This project is licensed under the MIT License.
