using uniffi.bytes_intense; 

class Program
{
    static void Main(string[] args)
    {
        Console.WriteLine("Hello, World!");
    
        // // Example of calling a Rust function that adds two numbers and returns the result
        uint addResult = BytesIntenseMethods.Add(5, 10);
        Console.WriteLine($"Result of Add(5, 10): {addResult}");

        // Example of calling a Rust function that returns a byte array
        byte[] returnedBuffer = BytesIntenseMethods.ReturnVec();
        Console.WriteLine($"ReturnVec returned a byte array of length: {returnedBuffer.Length}");

        byte[] dataToChange = [1, 2, 3, 4, 5];
        BytesIntenseMethods.TakeVecAndTryMutate(dataToChange);
        Console.WriteLine($"PushSomeData changed the data to: {string.Join(", ", dataToChange)}");

        String someString = "Hello from C#!";
        BytesIntenseMethods.TakeString(someString);

        var hello_from_rust = BytesIntenseMethods.ReturnString();
        Console.WriteLine($"ReturnString returned: {hello_from_rust}");
    }
}

