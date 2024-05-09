namespace Visitor;


public class Foo(string name, string other) : IAcceptor(name, other)
{
    public override void Accept(IVisitor accept)
    {
        accept.Do(this);
    }
}