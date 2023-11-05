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
>> * **Let the user know it's okay**: If it skips the last mentioned step, let the user and the system know everything was fine by returning an **Ok(0)**.