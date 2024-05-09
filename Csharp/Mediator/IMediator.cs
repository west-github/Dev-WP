namespace Mediator;

public interface IMediator
{
    void Notify(object sender, string ev);
}


public class Client : IMediator
{
    public OneComponent One { get; }

    public TwoComponent Two { get; }

    public Client()
    {
        this.One = new OneComponent(this);
        this.Two = new TwoComponent(this);
    }

    public void Notify(object sender, string ev)
    {
        if (ev == "One")
        {
            Console.WriteLine("One is called We can Call other function to react");
            One.Reaction();
        }

        if (ev == "Two")
        {
            Console.WriteLine("Two is called We can Call other function to react");
            Two.Reaction();
        }
    }
}