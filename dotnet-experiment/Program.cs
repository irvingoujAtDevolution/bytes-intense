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
    }
}

