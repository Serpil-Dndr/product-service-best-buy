use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "Best Buy 4K Smart TV".to_string(),
            price: 499.99,
            description: "Enjoy stunning 4K UHD resolution with the Best Buy Smart TV. Featuring a sleek design, built-in streaming apps, and voice control, it's perfect for movie nights and binge-watching your favorite shows.".to_string(),
            image: "/bestbuy-tv.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Best Buy Bluetooth Headphones".to_string(),
            price: 89.99,
            description: "Experience rich sound and deep bass with Best Buy's Bluetooth headphones. These over-ear headphones offer comfort, long battery life, and a noise-cancelling feature for an immersive listening experience.".to_string(),
            image: "/bestbuy-headphones.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Best Buy Wireless Speaker".to_string(),
            price: 129.99,
            description: "Take your music everywhere with the Best Buy Wireless Speaker. Waterproof and portable, it delivers crystal-clear sound whether you're at the beach or in your living room.".to_string(),
            image: "/bestbuy-speaker.jpg".to_string()
        },
        Product {
            id: 4,
            name: "Best Buy Smart Home Hub".to_string(),
            price: 99.99,
            description: "Control all your smart home devices with the Best Buy Smart Home Hub. Compatible with Alexa, Google Assistant, and Apple HomeKit, it makes managing your home easier than ever.".to_string(),
            image: "/bestbuy-homehub.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Best Buy Wireless Mouse".to_string(),
            price: 19.99,
            description: "Upgrade your workspace with Best Buy's wireless mouse. Featuring ergonomic design, precision tracking, and long battery life, it's perfect for both home and office use.".to_string(),
            image: "/bestbuy-mouse.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Best Buy 3D Printer".to_string(),
            price: 349.99,
            description: "Bring your ideas to life with the Best Buy 3D Printer. Easy to set up and use, this printer is perfect for hobbyists and creators looking to create detailed, high-quality prints.".to_string(),
            image: "/bestbuy-3dprinter.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Best Buy Smartwatch".to_string(),
            price: 199.99,
            description: "Stay connected and track your health with the Best Buy Smartwatch. Featuring fitness tracking, heart rate monitoring, and notifications, it's designed to keep you active and on top of your day.".to_string(),
            image: "/bestbuy-smartwatch.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Best Buy Home Security Camera".to_string(),
            price: 89.99,
            description: "Protect your home with the Best Buy Home Security Camera. Featuring HD video, night vision, and motion detection, it ensures peace of mind whether you're at home or away.".to_string(),
            image: "/bestbuy-securitycamera.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Best Buy Electric Grill".to_string(),
            price: 59.99,
            description: "Grill your favorite foods indoors with the Best Buy Electric Grill. With adjustable temperature control and non-stick plates, it's perfect for cooking without the smoke and mess.".to_string(),
            image: "/bestbuy-grill.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Best Buy Cordless Vacuum Cleaner".to_string(),
            price: 179.99,
            description: "Keep your home spotless with the Best Buy Cordless Vacuum Cleaner. With powerful suction, long battery life, and a lightweight design, it makes cleaning effortless.".to_string(),
            image: "/bestbuy-vacuum.jpg".to_string()
        }
        
    ]
}