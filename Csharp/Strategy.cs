namespace Strategy;
#pragma warning disable IDE1006 // Naming Styles

public record Invoice(string Name, decimal Amount);

public interface IPayment
{
    void Pay(Invoice request);
}

public class PayPal : IPayment
{
    public void Pay(Invoice request)
    {
        Console.WriteLine("PAYING with paypal");
    }
}

public class Google : IPayment
{
    public void Pay(Invoice request)
    {
        Console.WriteLine("PAYING with google");
    }
}

public class Client(IPayment payment)
{
    private IPayment Payment
    {
        get => payment;
    }


    public void order(Invoice invoice)
    {
        Console.WriteLine("Before Paying");

        Payment.Pay(invoice);

        Console.WriteLine("After Paying");
    }
}