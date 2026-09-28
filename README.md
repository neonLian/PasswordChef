# PasswordChef
Wordlist / password candidate generator using step-by-step recipes; supports checking all permutations of words, cycling through upper/lowercase modifiers, and more

## Usage

Prints all password candidates to stdout
```
./PasswordChef.exe --recipe recipe.txt
```
If you want to use the password candidates to crack hashes, you can pipe the output into a password cracking program like [Hashcat](https://github.com/hashcat/hashcat) or [John the Ripper](https://github.com/openwall/john).

A recipe file should have a [list of PasswordChef recipe steps](#recipe-steps-list) separated by new lines.

## Example 1
recipe.txt
```
wordlist words.txt
mask d
```
words.txt
```
hunter
gatherer
```
PasswordChef output:
```
hunter1
hunter2
hunter3
...
hunter8
hunter9
gatherer1
gatherer2
...
gatherer7
gatherer8
gatherer9
```

## Example 2
recipe.txt, same words.txt as example 1
```
wordlist#word words.txt
constant -
dup #word
mask ds
```
Output:
```
hunter-hunter1!
hunter-hunter1"
...
hunter-hunter9~
gatherer-gatherer1!
...
gatherer-gatherer9~
```

## Example 3
recipe.txt
```
constant+ult hunter
constant 2
rearrange #1 #2
```
Output:
```
HUNTER2
hunter2
Hunter2
2HUNTER
2hunter
2Hunter
```

## Example 4
recipe.txt
```
constant gatherer
replace #1 a4 A4 e3 E3 l1 L1 s5 S5 t7 T7
```
Output:
```
gatherer
g4therer
gath3rer
gather3r
ga7herer
```


## Recipe steps list

### `wordlist` 
Output each word in a wordlist file
```
wordlist words.txt
```

### `mask` and `maskinc`
Go through each combination of characters, based on character type

L = letters, d = digits, s = special, l = lowercase, u = uppercase, e = everything

`maskinc` will start from 1st character only, then do first 2 characters, until reaching all characters
```
mask ds
# outputs 1! 1" 1# ... 9~
```
```
maskinc ull
# outputs A B C D ... Aa Ab Ac ... Aaa Aab ... Zzy Zzz
```

### `constant`
Constant text that doesn't change (except for modifiers)
```
constant whatever-you-want!
# outputs whatever-you-want!
```

### `duplicate`
Duplicate a previous step text, by [ID](#id)
```
mask d
duplicate #1
# outputs 11 22 33 44 55 66 77 88 99
```

### `rearrange`
Go through all permutations of a list of previous steps, by [ID](#id) or [class](#class)
```
constant Apple
constant Pen
constant Pineapple
rearrange #2 #3
# outputs ApplePenPineapple ApplePineapplePen
```
```
constant.list Apple
constant.list Pen
constant.list Pineapple
rearrange .list
# outputs ApplePenPineapple ApplePineapplePen PenApplePineapple PenPineappleApple PineappleApplePen PineapplePenApple
```

### `concat`
Combine multiple steps into a single step

Useful when paired with `rearrange` or modifiers
```
constant apple
constant pen
constant pineapple
concat+t #1 #2
rearr #3 #4
# outputs pineappleApplepen Applepenpineapple
```

### `replace`
Replace letters, one at a time
```
constant gatherer
replace #1 a4 A4 e3 E3 l1 L1 s5 S5 t7 T7
# outputs gatherer g4therer gath3rer gather3r ga7herer
```
If you want to replace multiple characters at the same time, try use multiple `replace` steps
```
constant bees
replace #1 e3 s5
replace #2 e3 s5
# outputs bees b3es be3s bee5 b3es b33s b3e5 be3s b33s be35 bee5 b3e5 be35
```

### `anycase`
Go through all possible upper/lowercase combinations of a step
```
constant abc
anycase #1
# outputs abc abC aBc aBC Abc AbC ABc ABC
```

### `firstn`, `lastn`, `deletefirstn`, `deletelastn`
`firstn` keeps the first `n` letters
```
wordlist words.txt  # contains crustacean tachibana indubitably
firstn #1 5
# outputs crust tachi indub
```
`lastn` keeps the last `n` letters
```
wordlist words.txt  # contains crustacean tachibana indubitably
lastn #1 5
# outputs acean ibana tably
```
`deletefirstn` deletes the first `n` letters
```
wordlist words.txt  # contains crustacean tachibana indubitably
deletefirstn #1 5
# outputs acean bana itably
```
`deletelastn` deletes the last `n` letters
```
wordlist words.txt  # contains crustacean tachibana indubitably
deletelastn #1 5
# outputs crust tach indubi
```

## Recipe modifiers list
### Case
Adding `+` will change case: t = title, u = all upper, l = lowercase, o = original case
```
wordlist+ult words.txt
```

### Optional
`?` will make a step optional 
```
constant hunter
constant? 2
# outputs hunter hunter2
```

### ID
Adding `#ID` (`ID` can be anything) will allow step to be referenced later
```
wordlist#word1 words.txt
wordlist#word2 words.txt
rearrange #word1 #word2
```

All steps will have default ID equal to step number
```
wordlist list.txt
duplicate #1
```

### Class
Adding `.class` (`class` can be anything) will allow step to be referenced later, does not need to be unique
```
wordlist.word list.txt
wordlist.word list.txt
rearrange .word
```

### Hide from output
`^` hides a step from directly outputting but allows referencing by other steps
```
constant^ AAA      # does not output
constant^ BBB      # does not output
concat #1 #2       # outputs "AAABBB"
```

## Behaviors to be aware of

### If a step is referenced by a later step, it is no longer included in the output
```
constant Apple
constant Pen
concat #1 #2
# outputs "ApplePen" instead of "ApplePenApplePen"
```
Exception is the `duplicate` step
```
constant Apple
constant Pen
duplicate #1
# outputs ApplePenApple
```

### Modifiers, `rearrange`, and `replace` always include a version with no modification
```
constant hunter
replace #1 e3
# outputs hunter hunt3r
```

## Downloads

See the Releases tab.
