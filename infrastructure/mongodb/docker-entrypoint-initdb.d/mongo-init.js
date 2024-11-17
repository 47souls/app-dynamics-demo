db.createUser({
    user: 'admin',
    pwd: 'admin',
    roles: [
        {
            role: 'readWrite',
            db: 'prediction',
        },
        {
            role: 'readWrite',
            db: 'binance',
        },
    ],
});

db = new Mongo().getDB("ordering");
db.createCollection('order_history');
