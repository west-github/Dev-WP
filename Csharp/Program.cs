using Mediator;
using Visitor;


IAcceptor[] acceptor_lists = [new Bar("West", "Tequila"), new Foo("West", "No")];
ConsoleVisitor visitor = new();

foreach (var item in acceptor_lists)
{
    item.Accept(visitor);
}

