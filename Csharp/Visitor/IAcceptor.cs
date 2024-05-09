namespace Visitor;

public abstract class IAcceptor(string name, string other)
{
    public string Name { get; } = name;
    public string Other { get; } = other;
    public abstract void Accept(IVisitor accept);
}