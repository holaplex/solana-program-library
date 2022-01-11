import * as anchor from '@project-serum/anchor';
import { Program } from '@project-serum/anchor';
import { AuctionHouseAgreement } from '../target/types/auction_house_agreement';

describe('auction-house-agreement', () => {

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.Provider.env());

  const program = anchor.workspace.AuctionHouseAgreement as Program<AuctionHouseAgreement>;

  it('Is initialized!', async () => {
    // Add your test here.
    const tx = await program.rpc.initialize({});
    console.log("Your transaction signature", tx);
  });

  describe('create approval', () => {
    it('results in an open approval');
    it('only succeeds when signed by auction house authority');
  });

  describe('approve pending approval', () => {
    it('approves the listing request');
    it('only succeedes when authorizer signs');
  });

  describe('delete agreement', () => {
    it('closes the agreement account')
    it('only invokable by the auction house authority')
  });

  describe('revoke agreement', () => {
    it('removes the approval of the agreement')
    it('only invokable by the authority of the agreement')
  });
});
