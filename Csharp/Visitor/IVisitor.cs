namespace Visitor;

public interface IVisitor
{
    public void Do(IAcceptor acceptor);
}

public class ConsoleVisitor : IVisitor
{
    public void Do(IAcceptor acceptor)
    {
        Console.WriteLine($"Visitor Implementation Using Acceptor With Name: {acceptor.Name} Other: {acceptor.Other}");
    }
}