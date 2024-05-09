namespace Mediator;
#pragma warning disable IDE1006 // Naming Styles

public class Component(IMediator mediator)
{
    public IMediator Mediator
    {
        get; set;
    } = mediator;

    public void Reaction()
    {
        Console.WriteLine($"This is a reaction from mediator: {this}");
    }
}


public class OneComponent(IMediator mediator) : Component(mediator)
{
    public void Called()
    {
        Console.WriteLine("One is Called Let see what will happen");
        this.Mediator.Notify(this, "One");
    }
}



public class TwoComponent(IMediator mediator) : Component(mediator)
{
    public void Called()
    {
        Console.WriteLine("Two is Called Let see what will happen");
        this.Mediator.Notify(this, "Two");
    }
}

