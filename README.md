# Rusty Hangman
<img src="image.png" width="350">  

This is a simple command-line implementation of the classic Hangman game written in Rust. The program generates a random word and allows the player to guess letters within a limited number of chances.


## Features
- **Random word generation** using the random_word crate.
- Displays hidden letters as underscores (_) to represent the word.
- Tracks the player's guesses and updates the word with correct letters.
- Limits the number of incorrect guesses to 7 attempts.
- Displays appropriate messages for correct guesses, incorrect guesses, victory, or defeat.
  
  
## How It Works 📢
1. A random word is generated at the start of the game.
2. The player is shown the word in a hidden form, where each letter is replaced by _.
3. The player inputs one letter at a time.
    - If the guessed letter exists in the word, it is revealed in its appropriate position(s).
    - If the guessed letter does not exist in the word, the player loses one of their 7 chances.
4. The game ends when:
    - The player guesses the entire word correctly.
    - The player uses all 7 chances without guessing the word.

## Installation ⚙️

To run this program on your machine, follow these steps:

1. **Clone the repository**:  
   ```git clone https://github.com/mzums/rusty_hangman```
2. **Navigate to the project directory**:  
    ```cd rusty_hangman```
3. **Build and run the program**:  
    ```cargo run```

## Contributing 🖋️
Contributing is always welcome!  
Steps to contribute:
1. Fork the repository.
2. Create a new branch  
    ```git checkout -b feature/your-feature```
3. Make your changes and commit them.  
    ```git commit -am 'Add new feature'```
4. Push to your branch.  
    ```git push origin feature/your-feature```
5. Create a new Pull Request.
