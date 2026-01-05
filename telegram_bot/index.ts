import TelegramBot from "node-telegram-bot-api";
import { Connection, PublicKey } from "@solana/web3.js";

const connection = new Connection("https://api.mainnet-beta.solana.com");

const token = process.env.BOT_TOKEN;

if (!token) {
  throw new Error("BOT TOKEN NOT PROVIDED");
} else {
  const bot = new TelegramBot(token, { polling: true });

  //onText this will check for the /command one we have to put the logic inside them
  bot.onText(/\/start/, (msg) => {
    // bot.sendMessage(msg.chat.id, "Welcome",

    // reply_markup: {
    //   keyboard: [[/balance, /apsw], ["hi"], ["bye"]],
    // },
    bot.sendMessage(msg.chat.id, "Welcome", {
      reply_markup: {
        keyboard: [
          [{ text: "/start" }, { text: "/balance" }],
          [{ text: "hi" }],
          [{ text: "bye" }],
        ],
      },
    });
  });
  bot.onText(/\/balance/, (msg,match) => {
  if(match == null) return bot.sendMessage(msg.chat.id, "Provide the Key")
    bot.sendMessage(msg.chat.id, "Your Balance is 1 Billion Euro");
const walletAddress = match[1]
    try {
     const pubKey = new PublicKey(walletAddress!) 
     const balance = 
    } catch (error) {
      
    }
  bot.onText(/\/sendpic/, (msg) => {
    bot.sendPhoto(msg.chat.id, "https://www.somesite.com/image.jpg", {
      caption: "Here we go ! \nThis is just a caption ",
    });
  });

  // Listen for any kind of message. There are different kinds of
  // messages.
  bot.on("message", (msg) => {
    const chatId = msg.chat.id;
    let Hi = "hi";
    let bye = "bye";
    if (msg.text?.toString().toLowerCase().indexOf(Hi) === 0) {
      bot.sendMessage(chatId, "Hello how can i help you");
      return;
    } else if (msg.text?.toString().toLowerCase().includes(bye)) {
      // use includes if anything containg the bye word we send this message back
      return bot.sendMessage(chatId, "Hope i help see you around, Bye");
    }
    bot.sendMessage(chatId, "Received your message");
  });
}

