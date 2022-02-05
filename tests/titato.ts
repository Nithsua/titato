import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { Titato } from '../target/types/titato';

describe('titato', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.Titato as Program<Titato>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });
});
