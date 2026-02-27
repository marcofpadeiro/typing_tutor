# clack
Rust type tutor in cli to help me learn colemak :) 

## installation

```bash
git clone https://github.com/marcofpadeiro/clack
cd clack

cargo install --path .
```

## usage

run the game with default settings (30s timer, medium dictionary):
```bash
clack
```

#### command line arguments
you can customize your session using flags:
| flag | short | default | description |
| :--- | :--- | :--- | :--- |
| `--mode` | `-m` | `timer` | **timer:** fixed time, variable words <br>**words:** fixed words, variable time |
| `--quantity` | `-q` | `30` | seconds for timer word count for words |
| `--dictionary` | `-d` | `medium` | **predefined:** small, medium, long<br>**custom:** local path or url |
| `--filter` | `-f` | `none` | restrict dictionary to words containing only these characters |
| `--word-preview` | `-w` | `3` | number of upcoming words to show in the queue |
| `--min-word-size` | `-s` | `2` | exclude words shorter than this length |

## examples
##### race against time
type as many words as you can in one minute:
```bash
clack --mode timer --quantity 60
```

#### accuracy practice
type 50 words as fast as possible using the small dictinary:
```bash
clack --mode words --quantity 50 --dictionary short
```

#### practice home row
only type words that consist of home row characters:
```bash
clack --filter "asdfghjkl"
```

#### advanced
type 67 words, with no word preview, include words of any size, using a dictionary from a url
```bash
clack --mode words --quantity 67 --word-preview 0 --min-word-size 0 --dictionary "https://raw.githubusercontent.com/first20hours/google-10000-english/refs/heads/master/20k.txt"
```

## roadmap
- [x] Basic Word reading and print
- [x] Timer
- [x] Option to choose between **timed runs** vs **amount of words** 
- [x] Accuracy tracker
- [x] Dictionary implementation
- [x] Proper cli flags
- [ ] Live WPM counter
- [x] Support for custom dictionary files
- [x] Support for filtering dictionary words based on provided letters
- [ ] Variable number_words_show if 0 show all the words
- [ ] Log runs
- [ ] Add ,.!; symbols to words that appear
- [ ] Option to filter out words containg '
- [ ] Option to add numbers to rotation
- [ ] Option to select different difficulties of the words that are shown
- [ ] Option to appear upper case letters
