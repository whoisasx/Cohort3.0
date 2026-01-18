#!/usr/bin/env node

import chalk from "chalk";
import boxen from "boxen";
import figlet from "figlet";
import gradient from "gradient-string";

console.log(
    gradient.passion(figlet.textSync("whoisasx", { horizontalLayout: "full" }))
);

const adilCard = `
${chalk.cyan.bold("👋 Yo, I'm Adil Shaikh")}

🔧  ${chalk.bold(
    "Role:"
)} Fullstack Dev | DSA & C++ Solver | Web3 Curious | DevOps Explorer
🚧  ${chalk.bold("Project:")} Building ${chalk.greenBright(
    "Synk — Your thoughts, structured."
)}
📫  ${chalk.bold("Email:")} connect.asxcode@gmail.com
🌐  ${chalk.bold("Website:")} https://asxcode.com
🔗  ${chalk.bold("LinkedIn:")} https://linkedin.com/in/adilshaikh4064

💬  ${chalk.italic(
    `“Code is just structured thought. I’m learning to think better.”`
)}
`;

console.log(
    boxen(adilCard, {
        padding: 1,
        margin: 1,
        borderStyle: "round",
        borderColor: "greenBright",
    })
);
