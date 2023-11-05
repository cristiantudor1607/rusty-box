**Name: Tudor Cristian-Andrei**

**Group: 321CAa**

## <font color="#39A7FF"> Rustybox </font>

### Structure of the code
I choose to implement each command on it's separate file name <font color="#87C4FF"> **#command_name** </font>.rs. There is an additional file, named <font color="#87C4FF">utils.rs</font>, with some general-purpose functions that can be used everywhere in the project. There are some functions, like <font color="#E0F4FF">create_newdir</font> from <font color="#87C4FF">mkdir.rs</font>, that aren't included in the <font color="#87C4FF">utils.rs</font>, but are "borrowed" by other files. I didn't consider them general-purpose, so I didn't include them in the <font color="#87C4FF">utils.rs</font>.
> Each file contains at least one public function, which will be called from main, to execute the command. If the other functions aren't used for another command, they aren't public. I don't need them to be public.
>> Each public function that is used in main follow a specific pattern:
>> * **The return type**: **Result< i32, () >**: Maybe it doesn't make sens to use **()** as error, but I didn't want to get lost in numbers (and I used i32 to much in C and i wanted something new). It can easly be replaced with an **Option< i32 >**, or just an **i32**, but using **Result**, I can highlight better that there was an error at some point, and isn't the user fault. If the command was succesfully done, or there is an invalid input provided, a code is returned to main through **Ok**, because nothing bad happened at system level. Everything worked fine, despite the fact that the machine did nothing.
>> * **First things first**: Check if the user knows how to use the terminal and it's features. If the command require arguments or options, the first thing to do is to verify the arguments. If they aren't a valid input, return a **-1**.
>> * **Do your tasks**: Use your functions to do what the user requested.
>> * **Let the user know why it didn't work**: I tried to avoid using **unwrap** or **?**, and I used a similar approach to the 2nd laboratory, with **error propagation**. If an error is happening in a function, the error is send to the caller, which is send to the caller, ... and so on, until it reaches the function that is called in main. Here, the error is printed using **eprintln!** macro, and it return an **Err( () )** to let the **main** know that something was wrong at system level.
>> * **Let the user know it's okay**: If it skips the last mentioned step, let the user and the system know everything was fine by returning an **Ok(0)**. <br>

### The Implementation of The Commands
1. <font color="#87C4FF">**pwd**</font>: I used the <font color="#00A9FF">current_dir</font> function from <font color="#0174BE">std::env</font>, which returns the current path. This command is straight forward and I don't have much to explain. There is an additional match on the result of <font color="#00A9FF">current_dir</font>, in case it returns **None**.
2.  <font color="#87C4FF">**echo**</font>: Maybe using <font color="#00A9FF">write!</font> macro seems odd. The real reason why I choose to use this is because it can fail, and it says that in case of failure echo returns -20. If I used **println!**, I didn't find anything about it failing. I don't know how, I don't know why. Because it's my first time with Rust, I didn't know how to use write! and I looked for a method at link [1].
3. <font color="#87C4FF">**cat**</font>: For this, I wanted to replicate every behaviour of the linux terminal command. As an example, when the user type *"cat"* and press Enter, it will dive into an infinte loop, that can be closed only by a signal. As above, I used <font color="#0174BE"> io::stdin().read_line</font>, after looking at link [2]. I think it is worth mentioning that if a file doesn't exist and it will return an error, it continues for the rest of the files, and drops the error at the end.
4. <font color="#87C4FF">**mkdir**</font>: Thanks Rust for having <font color="#0174BE">std::fs::DirBuilder</font>. I used this module for creating the directory (the <font color="#00A9FF">create()</font> method). This part was straight forward. In case of errors, there are printed specific messages to stderr.
5. <font color="#87C4FF">**mv**</font>: No matter for what is used, mv is just a rename. Basically, I check if the file exists, and if it does, I rename it using <font color="#00A9FF">rename()</font> method from <font color="#0174BE">std::fs</font>.
6. <font color="#87C4FF">**ln**</font>: First, I determine the type of the link, soft or hard. The links are created using <font color="#00A9FF">hard_link()</font> function from <font color="#0174BE">std::fs</font> or using <font color="#00A9FF">symlink()</font> function from <font color="#0174BE">std::os::unix::fs</font>. This time I don't check if the entry exists, because the functions won't create the link and will throw the error.
7. <font color="#87C4FF">**rmdir**</font>: The command consist of one checking, if the entry is a directory, but I think it could have been skipped, and a standard library call. I used <font color="#00A9FF">remove_dir()</font> which throw an error anyway, if the path doesn't exist, but I wanted custom messages for errors.
8. <font color="#87C4FF">**rm**</font>: This command was a little more complex than the ones mentioned above. First, I check if the user provided some options, using a structure which I will talk about later. After that, I check if the arguments provided are good. If not, return the invalid command code. In the last part, I switch between 3 functions, depending on the options: <font color="#00A9FF">remove_file()</font>, <font color="#00A9FF">remove_dir()</font> and <font color="#00A9FF">remove_dir_all()</font>. It is worth mentioning that the error propagation is a little bit different in this case.
9. <font color="#87C4FF">**ls**</font>: About the options and the arguments, it's a similar approach to rm. This things are better explained in the code, so basically, after all the required verifications, I load the directory in a <font color="#0174BE">ReadDir</font> variable, and go through all of it's contents. In the recursive case, I call the function again, for every entry. The recursive call stops when it reaches a file.
10. <font color="#87C4FF">**cp**</font>: It is worth talking about the recursive case. Similar approach to ls, but I had to parse some filenames (better see the code) before going into recursion, and creating some directories if neccessary. The basic copy functionality relies on the <font color="#00A9FF">copy()</font> function from <font color="#0174BE">std::fs</font>.
11. <font color="#87C4FF">**touch**</font>: For updating the acces time, I open the file and read it's content. For updating the modification time, I open the file for append and I write an extra 0u8 byte at the end, that doesn't matter. If the file doesn't exists, I use the <font color="#00A9FF">create()</font> function.
12. <font color="#87C4FF">**chmod**</font>: Maybe chmod was the hardest one, because I had to work with numbers in base 8. After even more checkings and setting flags, I get the current permissions, and perform operations on them. It would be pointless to explain the operations, because the code says it all. It was really hard to explain them even in comments. I used link [3] to learn how to set permissions.
13. <font color="#87C4FF">**grep**</font>: This bonus was really easy, because the  <font color="#0174BE">regex</font> crate gave us <font color="#00A9FF">is_match()</font> method, which can be applied on a Regex variable to check if a string has a pattern.

### Structures
> They are used in a limited scope, each with it's own command, and there isn't something complex about them

1. **PathStatus**: it is used to fit a path in a category: Directory, File, Or it doesn't exist
```rust
pub enum PathStatus {
    IsDir,
    IsFile,
    IsNot,
}
```

2. **RmOption**: the flags for rm command: recursive, only the empty directories and files, only files, or all
```rust
pub enum RmOption {
    Recursive,
    EmptyDirs,
    OnlyFiles,
    All,
}
```

3. **TouchDateTyoe**: the flags for touch command: modification date, access date, the default creation flag, or no creation flag
```rust
pub enum TouchDateType {
    Modify,
    Access,
    Create,
    NoCreate,
}
```

4. **LinkType**: describes what type of link should be created, hard or soft
```rust
pub enum LinkType {
    SoftLink,
    HardLink,
}
```

5. **CpOption**: describes if the recursive option for cp is active
```rust
pub enum CpOption {
    Recursive,
    NonRecursive,
}
```

6. **ListingType**: the flags for ls: recursive, all entries, even the ones hidden, these two combined, or the default

```rust
pub enum ListingType {
    Default,
    Recursive,
    All,
    RecursiveAll,
}
```

7. **UserType**: describes what types of user should be chmod applied to

```rust
pub enum UserType {
    ForOwner,
    ForGroup,
    ForOthers,
    ForAll,
}
```

8. **ChmodTyoe**: describes the type of chmod, remove or add permissions

```rust
pub enum ChmodType {
    Add,
    Del,
}
```
### Links
[1] https://doc.rust-lang.org/std/macro.print.html <br>
[2] https://fitech101.aalto.fi/programming-languages/rust/8-interaction-input-and-os/#:~:text=To%20read%20user%20input%20in,written%20on%20the%20command%20line
[3] https://doc.rust-lang.org/std/os/unix/fs/trait.PermissionsExt.html

