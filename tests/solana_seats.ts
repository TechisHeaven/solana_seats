import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaSeats } from "../target/types/solana_seats";

describe("solana_seats", () => {
  // Configure the client to use the local cluster.
  const event = anchor.web3.Keypair.generate();
  const provider = anchor.AnchorProvider.env(); 

  anchor.setProvider(provider);
  
  const program = anchor.workspace.SolanaSeats as Program<SolanaSeats>;

    const organizer = provider.wallet.publicKey;

  it("Create a Event!", async () => {
    // Add your test here.
    await program.methods.createEvent(new anchor.BN(10))
      .accounts({
        event: event.publicKey,
        organizer: organizer,
      })
      .signers([event])
      .rpc();
      
    const eventAccount = await program.account.event.fetch(event.publicKey);
    console.log("Event Created with total tickets:", eventAccount.totalTickets.toString());
  });


  it("Buys a ticket", async () => {
    const event = anchor.web3.Keypair.generate();
    const user = anchor.web3.Keypair.generate();


    await program.methods.buyTicket().accounts({
        event: event.publicKey,
        user: user.publicKey,
      })
      .signers([user])
      .rpc();

    const userAccount = await program.account.user.fetch(user.publicKey);
    
    console.log("User bought a ticket. Total tickets owned:", userAccount.ticketOwned.toString());
  });


});




