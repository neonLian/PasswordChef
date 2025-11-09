# PasswordChef
Wordlist / password candidate generator using step-by-step recipes; supports checking all permutations of words, cycling through upper/lowercase modifiers, and more

## Usage

Print all password candidates to stdout
```
./PasswordChef.exe --recipe recipe.txt
```

The recipe file should have a list of recipe steps separated by new lines.
(Comments are included below to explain but currently comments are not supported in recipes.)

Recipe steps examples and explanation:
```
# Go through each word in a wordlist file
wordlist words.txt

# Go through each combination of characters, based on character type
# L = letters, d = digits, s = special, l = lowercase, u = uppercase, e = everything
mask ds
maskinc ull

# Constant text that doesn't change
constant xx

# Duplicate a previous step text, by ID 
duplicate #3

# Go through all orderings of a list of previous steps
rearrange #2 #3
rearrange .list

# Combine multiple steps into a single step
concat #2 #3
concat .list

# Replace letters, one at a time
replace #2 a4 A4 e3 E3 l1 L1 s5 S5 t7 T7
```

Steps can have modifiers:
```
# Adding + will change case: t = title, u = all upper, l = lowercase, o = original case
wordlist+ult words.txt

# ? will make it optional 
# Below example will generate both with and without "123"
constant? 123 

# Adding #ID will allow step to be referenced later
wordlist#word1 words.txt
wordlist#word2 words.txt

# All steps will have default ID equal to step number
wordlist list.txt
dup #1

# Then you can reference the ID later (make sure to include a space)
duplicate #word1

# Also can use .class, does not need to be unique
wordlist.word list.txt
wordlist.word list.txt
rearrange .word

# ^ hides from output but allows referencing by other steps
constant^ AAA      # does not output
constant^ BBB      # does not output
concat #1 #2       # outputs "AAABBB"

```

## Downloads

See the Releases tab.
