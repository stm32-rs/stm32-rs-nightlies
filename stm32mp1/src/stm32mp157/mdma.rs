#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gisr0: GISR0,
    _reserved1: [u8; 0x04],
    sgisr0: SGISR0,
    _reserved2: [u8; 0x34],
    c0isr: C0ISR,
    c0ifcr: C0IFCR,
    c0esr: C0ESR,
    c0cr: C0CR,
    c0tcr: C0TCR,
    c0bndtr: C0BNDTR,
    c0sar: C0SAR,
    c0dar: C0DAR,
    c0brur: C0BRUR,
    c0lar: C0LAR,
    c0tbr: C0TBR,
    _reserved13: [u8; 0x04],
    c0mar: C0MAR,
    c0mdr: C0MDR,
    _reserved15: [u8; 0x08],
    c1isr: C1ISR,
    c1ifcr: C1IFCR,
    c1esr: C1ESR,
    c1cr: C1CR,
    c1tcr: C1TCR,
    c1bndtr: C1BNDTR,
    c1sar: C1SAR,
    c1dar: C1DAR,
    c1brur: C1BRUR,
    c1lar: C1LAR,
    c1tbr: C1TBR,
    _reserved26: [u8; 0x04],
    c1mar: C1MAR,
    c1mdr: C1MDR,
    _reserved28: [u8; 0x08],
    c2isr: C2ISR,
    c2ifcr: C2IFCR,
    c2esr: C2ESR,
    c2cr: C2CR,
    c2tcr: C2TCR,
    c2bndtr: C2BNDTR,
    c2sar: C2SAR,
    c2dar: C2DAR,
    c2brur: C2BRUR,
    c2lar: C2LAR,
    c2tbr: C2TBR,
    _reserved39: [u8; 0x04],
    c2mar: C2MAR,
    c2mdr: C2MDR,
    _reserved41: [u8; 0x08],
    c3isr: C3ISR,
    c3ifcr: C3IFCR,
    c3esr: C3ESR,
    c3cr: C3CR,
    c3tcr: C3TCR,
    c3bndtr: C3BNDTR,
    c3sar: C3SAR,
    c3dar: C3DAR,
    c3brur: C3BRUR,
    c3lar: C3LAR,
    c3tbr: C3TBR,
    _reserved52: [u8; 0x04],
    c3mar: C3MAR,
    c3mdr: C3MDR,
    _reserved54: [u8; 0x08],
    c4isr: C4ISR,
    c4ifcr: C4IFCR,
    c4esr: C4ESR,
    c4cr: C4CR,
    c4tcr: C4TCR,
    c4bndtr: C4BNDTR,
    c4sar: C4SAR,
    c4dar: C4DAR,
    c4brur: C4BRUR,
    c4lar: C4LAR,
    c4tbr: C4TBR,
    _reserved65: [u8; 0x04],
    c4mar: C4MAR,
    c4mdr: C4MDR,
    _reserved67: [u8; 0x08],
    c5isr: C5ISR,
    c5ifcr: C5IFCR,
    c5esr: C5ESR,
    c5cr: C5CR,
    c5tcr: C5TCR,
    c5bndtr: C5BNDTR,
    c5sar: C5SAR,
    c5dar: C5DAR,
    c5brur: C5BRUR,
    c5lar: C5LAR,
    c5tbr: C5TBR,
    _reserved78: [u8; 0x04],
    c5mar: C5MAR,
    c5mdr: C5MDR,
    _reserved80: [u8; 0x08],
    c6isr: C6ISR,
    c6ifcr: C6IFCR,
    c6esr: C6ESR,
    c6cr: C6CR,
    c6tcr: C6TCR,
    c6bndtr: C6BNDTR,
    c6sar: C6SAR,
    c6dar: C6DAR,
    c6brur: C6BRUR,
    c6lar: C6LAR,
    c6tbr: C6TBR,
    _reserved91: [u8; 0x04],
    c6mar: C6MAR,
    c6mdr: C6MDR,
    _reserved93: [u8; 0x08],
    c7isr: C7ISR,
    c7ifcr: C7IFCR,
    c7esr: C7ESR,
    c7cr: C7CR,
    c7tcr: C7TCR,
    c7bndtr: C7BNDTR,
    c7sar: C7SAR,
    c7dar: C7DAR,
    c7brur: C7BRUR,
    c7lar: C7LAR,
    c7tbr: C7TBR,
    _reserved104: [u8; 0x04],
    c7mar: C7MAR,
    c7mdr: C7MDR,
    _reserved106: [u8; 0x08],
    c8isr: C8ISR,
    c8ifcr: C8IFCR,
    c8esr: C8ESR,
    c8cr: C8CR,
    c8tcr: C8TCR,
    c8bndtr: C8BNDTR,
    c8sar: C8SAR,
    c8dar: C8DAR,
    c8brur: C8BRUR,
    c8lar: C8LAR,
    c8tbr: C8TBR,
    _reserved117: [u8; 0x04],
    c8mar: C8MAR,
    c8mdr: C8MDR,
    _reserved119: [u8; 0x08],
    c9isr: C9ISR,
    c9ifcr: C9IFCR,
    c9esr: C9ESR,
    c9cr: C9CR,
    c9tcr: C9TCR,
    c9bndtr: C9BNDTR,
    c9sar: C9SAR,
    c9dar: C9DAR,
    c9brur: C9BRUR,
    c9lar: C9LAR,
    c9tbr: C9TBR,
    _reserved130: [u8; 0x04],
    c9mar: C9MAR,
    c9mdr: C9MDR,
    _reserved132: [u8; 0x08],
    c10isr: C10ISR,
    c10ifcr: C10IFCR,
    c10esr: C10ESR,
    c10cr: C10CR,
    c10tcr: C10TCR,
    c10bndtr: C10BNDTR,
    c10sar: C10SAR,
    c10dar: C10DAR,
    c10brur: C10BRUR,
    c10lar: C10LAR,
    c10tbr: C10TBR,
    _reserved143: [u8; 0x04],
    c10mar: C10MAR,
    c10mdr: C10MDR,
    _reserved145: [u8; 0x08],
    c11isr: C11ISR,
    c11ifcr: C11IFCR,
    c11esr: C11ESR,
    c11cr: C11CR,
    c11tcr: C11TCR,
    c11bndtr: C11BNDTR,
    c11sar: C11SAR,
    c11dar: C11DAR,
    c11brur: C11BRUR,
    c11lar: C11LAR,
    c11tbr: C11TBR,
    _reserved156: [u8; 0x04],
    c11mar: C11MAR,
    c11mdr: C11MDR,
    _reserved158: [u8; 0x08],
    c12isr: C12ISR,
    c12ifcr: C12IFCR,
    c12esr: C12ESR,
    c12cr: C12CR,
    c12tcr: C12TCR,
    c12bndtr: C12BNDTR,
    c12sar: C12SAR,
    c12dar: C12DAR,
    c12brur: C12BRUR,
    c12lar: C12LAR,
    c12tbr: C12TBR,
    _reserved169: [u8; 0x04],
    c12mar: C12MAR,
    c12mdr: C12MDR,
    _reserved171: [u8; 0x08],
    c13isr: C13ISR,
    c13ifcr: C13IFCR,
    c13esr: C13ESR,
    c13cr: C13CR,
    c13tcr: C13TCR,
    c13bndtr: C13BNDTR,
    c13sar: C13SAR,
    c13dar: C13DAR,
    c13brur: C13BRUR,
    c13lar: C13LAR,
    c13tbr: C13TBR,
    _reserved182: [u8; 0x04],
    c13mar: C13MAR,
    c13mdr: C13MDR,
    _reserved184: [u8; 0x08],
    c14isr: C14ISR,
    c14ifcr: C14IFCR,
    c14esr: C14ESR,
    c14cr: C14CR,
    c14tcr: C14TCR,
    c14bndtr: C14BNDTR,
    c14sar: C14SAR,
    c14dar: C14DAR,
    c14brur: C14BRUR,
    c14lar: C14LAR,
    c14tbr: C14TBR,
    _reserved195: [u8; 0x04],
    c14mar: C14MAR,
    c14mdr: C14MDR,
    _reserved197: [u8; 0x08],
    c15isr: C15ISR,
    c15ifcr: C15IFCR,
    c15esr: C15ESR,
    c15cr: C15CR,
    c15tcr: C15TCR,
    c15bndtr: C15BNDTR,
    c15sar: C15SAR,
    c15dar: C15DAR,
    c15brur: C15BRUR,
    c15lar: C15LAR,
    c15tbr: C15TBR,
    _reserved208: [u8; 0x04],
    c15mar: C15MAR,
    c15mdr: C15MDR,
    _reserved210: [u8; 0x08],
    c16isr: C16ISR,
    c16ifcr: C16IFCR,
    c16esr: C16ESR,
    c16cr: C16CR,
    c16tcr: C16TCR,
    c16bndtr: C16BNDTR,
    c16sar: C16SAR,
    c16dar: C16DAR,
    c16brur: C16BRUR,
    c16lar: C16LAR,
    c16tbr: C16TBR,
    _reserved221: [u8; 0x04],
    c16mar: C16MAR,
    c16mdr: C16MDR,
    _reserved223: [u8; 0x08],
    c17isr: C17ISR,
    c17ifcr: C17IFCR,
    c17esr: C17ESR,
    c17cr: C17CR,
    c17tcr: C17TCR,
    c17bndtr: C17BNDTR,
    c17sar: C17SAR,
    c17dar: C17DAR,
    c17brur: C17BRUR,
    c17lar: C17LAR,
    c17tbr: C17TBR,
    _reserved234: [u8; 0x04],
    c17mar: C17MAR,
    c17mdr: C17MDR,
    _reserved236: [u8; 0x08],
    c18isr: C18ISR,
    c18ifcr: C18IFCR,
    c18esr: C18ESR,
    c18cr: C18CR,
    c18tcr: C18TCR,
    c18bndtr: C18BNDTR,
    c18sar: C18SAR,
    c18dar: C18DAR,
    c18brur: C18BRUR,
    c18lar: C18LAR,
    c18tbr: C18TBR,
    _reserved247: [u8; 0x04],
    c18mar: C18MAR,
    c18mdr: C18MDR,
    _reserved249: [u8; 0x08],
    c19isr: C19ISR,
    c19ifcr: C19IFCR,
    c19esr: C19ESR,
    c19cr: C19CR,
    c19tcr: C19TCR,
    c19bndtr: C19BNDTR,
    c19sar: C19SAR,
    c19dar: C19DAR,
    c19brur: C19BRUR,
    c19lar: C19LAR,
    c19tbr: C19TBR,
    _reserved260: [u8; 0x04],
    c19mar: C19MAR,
    c19mdr: C19MDR,
    _reserved262: [u8; 0x08],
    c20isr: C20ISR,
    c20ifcr: C20IFCR,
    c20esr: C20ESR,
    c20cr: C20CR,
    c20tcr: C20TCR,
    c20bndtr: C20BNDTR,
    c20sar: C20SAR,
    c20dar: C20DAR,
    c20brur: C20BRUR,
    c20lar: C20LAR,
    c20tbr: C20TBR,
    _reserved273: [u8; 0x04],
    c20mar: C20MAR,
    c20mdr: C20MDR,
    _reserved275: [u8; 0x08],
    c21isr: C21ISR,
    c21ifcr: C21IFCR,
    c21esr: C21ESR,
    c21cr: C21CR,
    c21tcr: C21TCR,
    c21bndtr: C21BNDTR,
    c21sar: C21SAR,
    c21dar: C21DAR,
    c21brur: C21BRUR,
    c21lar: C21LAR,
    c21tbr: C21TBR,
    _reserved286: [u8; 0x04],
    c21mar: C21MAR,
    c21mdr: C21MDR,
    _reserved288: [u8; 0x08],
    c22isr: C22ISR,
    c22ifcr: C22IFCR,
    c22esr: C22ESR,
    c22cr: C22CR,
    c22tcr: C22TCR,
    c22bndtr: C22BNDTR,
    c22sar: C22SAR,
    c22dar: C22DAR,
    c22brur: C22BRUR,
    c22lar: C22LAR,
    c22tbr: C22TBR,
    _reserved299: [u8; 0x04],
    c22mar: C22MAR,
    c22mdr: C22MDR,
    _reserved301: [u8; 0x08],
    c23isr: C23ISR,
    c23ifcr: C23IFCR,
    c23esr: C23ESR,
    c23cr: C23CR,
    c23tcr: C23TCR,
    c23bndtr: C23BNDTR,
    c23sar: C23SAR,
    c23dar: C23DAR,
    c23brur: C23BRUR,
    c23lar: C23LAR,
    c23tbr: C23TBR,
    _reserved312: [u8; 0x04],
    c23mar: C23MAR,
    c23mdr: C23MDR,
    _reserved314: [u8; 0x08],
    c24isr: C24ISR,
    c24ifcr: C24IFCR,
    c24esr: C24ESR,
    c24cr: C24CR,
    c24tcr: C24TCR,
    c24bndtr: C24BNDTR,
    c24sar: C24SAR,
    c24dar: C24DAR,
    c24brur: C24BRUR,
    c24lar: C24LAR,
    c24tbr: C24TBR,
    _reserved325: [u8; 0x04],
    c24mar: C24MAR,
    c24mdr: C24MDR,
    _reserved327: [u8; 0x08],
    c25isr: C25ISR,
    c25ifcr: C25IFCR,
    c25esr: C25ESR,
    c25cr: C25CR,
    c25tcr: C25TCR,
    c25bndtr: C25BNDTR,
    c25sar: C25SAR,
    c25dar: C25DAR,
    c25brur: C25BRUR,
    c25lar: C25LAR,
    c25tbr: C25TBR,
    _reserved338: [u8; 0x04],
    c25mar: C25MAR,
    c25mdr: C25MDR,
    _reserved340: [u8; 0x08],
    c26isr: C26ISR,
    c26ifcr: C26IFCR,
    c26esr: C26ESR,
    c26cr: C26CR,
    c26tcr: C26TCR,
    c26bndtr: C26BNDTR,
    c26sar: C26SAR,
    c26dar: C26DAR,
    c26brur: C26BRUR,
    c26lar: C26LAR,
    c26tbr: C26TBR,
    _reserved351: [u8; 0x04],
    c26mar: C26MAR,
    c26mdr: C26MDR,
    _reserved353: [u8; 0x08],
    c27isr: C27ISR,
    c27ifcr: C27IFCR,
    c27esr: C27ESR,
    c27cr: C27CR,
    c27tcr: C27TCR,
    c27bndtr: C27BNDTR,
    c27sar: C27SAR,
    c27dar: C27DAR,
    c27brur: C27BRUR,
    c27lar: C27LAR,
    c27tbr: C27TBR,
    _reserved364: [u8; 0x04],
    c27mar: C27MAR,
    c27mdr: C27MDR,
    _reserved366: [u8; 0x08],
    c28isr: C28ISR,
    c28ifcr: C28IFCR,
    c28esr: C28ESR,
    c28cr: C28CR,
    c28tcr: C28TCR,
    c28bndtr: C28BNDTR,
    c28sar: C28SAR,
    c28dar: C28DAR,
    c28brur: C28BRUR,
    c28lar: C28LAR,
    c28tbr: C28TBR,
    _reserved377: [u8; 0x04],
    c28mar: C28MAR,
    c28mdr: C28MDR,
    _reserved379: [u8; 0x08],
    c29isr: C29ISR,
    c29ifcr: C29IFCR,
    c29esr: C29ESR,
    c29cr: C29CR,
    c29tcr: C29TCR,
    c29bndtr: C29BNDTR,
    c29sar: C29SAR,
    c29dar: C29DAR,
    c29brur: C29BRUR,
    c29lar: C29LAR,
    c29tbr: C29TBR,
    _reserved390: [u8; 0x04],
    c29mar: C29MAR,
    c29mdr: C29MDR,
    _reserved392: [u8; 0x08],
    c30isr: C30ISR,
    c30ifcr: C30IFCR,
    c30esr: C30ESR,
    c30cr: C30CR,
    c30tcr: C30TCR,
    c30bndtr: C30BNDTR,
    c30sar: C30SAR,
    c30dar: C30DAR,
    c30brur: C30BRUR,
    c30lar: C30LAR,
    c30tbr: C30TBR,
    _reserved403: [u8; 0x04],
    c30mar: C30MAR,
    c30mdr: C30MDR,
    _reserved405: [u8; 0x08],
    c31isr: C31ISR,
    c31ifcr: C31IFCR,
    c31esr: C31ESR,
    c31cr: C31CR,
    c31tcr: C31TCR,
    c31bndtr: C31BNDTR,
    c31sar: C31SAR,
    c31dar: C31DAR,
    c31brur: C31BRUR,
    c31lar: C31LAR,
    c31tbr: C31TBR,
    _reserved416: [u8; 0x04],
    c31mar: C31MAR,
    c31mdr: C31MDR,
}
impl RegisterBlock {
    ///0x00 - MDMA global interrupt/status register
    #[inline(always)]
    pub const fn gisr0(&self) -> &GISR0 {
        &self.gisr0
    }
    ///0x08 - MDMA secure global interrupt/status register
    #[inline(always)]
    pub const fn sgisr0(&self) -> &SGISR0 {
        &self.sgisr0
    }
    ///0x40 - MDMA channel 0 interrupt/status register
    #[inline(always)]
    pub const fn c0isr(&self) -> &C0ISR {
        &self.c0isr
    }
    ///0x44 - MDMA channel 0 interrupt flag clear register
    #[inline(always)]
    pub const fn c0ifcr(&self) -> &C0IFCR {
        &self.c0ifcr
    }
    ///0x48 - MDMA channel 0 error status register
    #[inline(always)]
    pub const fn c0esr(&self) -> &C0ESR {
        &self.c0esr
    }
    ///0x4c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c0cr(&self) -> &C0CR {
        &self.c0cr
    }
    ///0x50 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c0tcr(&self) -> &C0TCR {
        &self.c0tcr
    }
    ///0x54 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c0bndtr(&self) -> &C0BNDTR {
        &self.c0bndtr
    }
    ///0x58 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c0sar(&self) -> &C0SAR {
        &self.c0sar
    }
    ///0x5c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c0dar(&self) -> &C0DAR {
        &self.c0dar
    }
    ///0x60 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c0brur(&self) -> &C0BRUR {
        &self.c0brur
    }
    ///0x64 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c0lar(&self) -> &C0LAR {
        &self.c0lar
    }
    ///0x68 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c0tbr(&self) -> &C0TBR {
        &self.c0tbr
    }
    ///0x70 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c0mar(&self) -> &C0MAR {
        &self.c0mar
    }
    ///0x74 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c0mdr(&self) -> &C0MDR {
        &self.c0mdr
    }
    ///0x80 - MDMA channel 1 interrupt/status register
    #[inline(always)]
    pub const fn c1isr(&self) -> &C1ISR {
        &self.c1isr
    }
    ///0x84 - MDMA channel 1 interrupt flag clear register
    #[inline(always)]
    pub const fn c1ifcr(&self) -> &C1IFCR {
        &self.c1ifcr
    }
    ///0x88 - MDMA channel 1 error status register
    #[inline(always)]
    pub const fn c1esr(&self) -> &C1ESR {
        &self.c1esr
    }
    ///0x8c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c1cr(&self) -> &C1CR {
        &self.c1cr
    }
    ///0x90 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c1tcr(&self) -> &C1TCR {
        &self.c1tcr
    }
    ///0x94 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c1bndtr(&self) -> &C1BNDTR {
        &self.c1bndtr
    }
    ///0x98 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c1sar(&self) -> &C1SAR {
        &self.c1sar
    }
    ///0x9c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c1dar(&self) -> &C1DAR {
        &self.c1dar
    }
    ///0xa0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c1brur(&self) -> &C1BRUR {
        &self.c1brur
    }
    ///0xa4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c1lar(&self) -> &C1LAR {
        &self.c1lar
    }
    ///0xa8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c1tbr(&self) -> &C1TBR {
        &self.c1tbr
    }
    ///0xb0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c1mar(&self) -> &C1MAR {
        &self.c1mar
    }
    ///0xb4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c1mdr(&self) -> &C1MDR {
        &self.c1mdr
    }
    ///0xc0 - MDMA channel 2 interrupt/status register
    #[inline(always)]
    pub const fn c2isr(&self) -> &C2ISR {
        &self.c2isr
    }
    ///0xc4 - MDMA channel 2 interrupt flag clear register
    #[inline(always)]
    pub const fn c2ifcr(&self) -> &C2IFCR {
        &self.c2ifcr
    }
    ///0xc8 - MDMA channel 2 error status register
    #[inline(always)]
    pub const fn c2esr(&self) -> &C2ESR {
        &self.c2esr
    }
    ///0xcc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c2cr(&self) -> &C2CR {
        &self.c2cr
    }
    ///0xd0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c2tcr(&self) -> &C2TCR {
        &self.c2tcr
    }
    ///0xd4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c2bndtr(&self) -> &C2BNDTR {
        &self.c2bndtr
    }
    ///0xd8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c2sar(&self) -> &C2SAR {
        &self.c2sar
    }
    ///0xdc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c2dar(&self) -> &C2DAR {
        &self.c2dar
    }
    ///0xe0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c2brur(&self) -> &C2BRUR {
        &self.c2brur
    }
    ///0xe4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c2lar(&self) -> &C2LAR {
        &self.c2lar
    }
    ///0xe8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c2tbr(&self) -> &C2TBR {
        &self.c2tbr
    }
    ///0xf0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c2mar(&self) -> &C2MAR {
        &self.c2mar
    }
    ///0xf4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c2mdr(&self) -> &C2MDR {
        &self.c2mdr
    }
    ///0x100 - MDMA channel 3 interrupt/status register
    #[inline(always)]
    pub const fn c3isr(&self) -> &C3ISR {
        &self.c3isr
    }
    ///0x104 - MDMA channel 3 interrupt flag clear register
    #[inline(always)]
    pub const fn c3ifcr(&self) -> &C3IFCR {
        &self.c3ifcr
    }
    ///0x108 - MDMA channel 3 error status register
    #[inline(always)]
    pub const fn c3esr(&self) -> &C3ESR {
        &self.c3esr
    }
    ///0x10c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c3cr(&self) -> &C3CR {
        &self.c3cr
    }
    ///0x110 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c3tcr(&self) -> &C3TCR {
        &self.c3tcr
    }
    ///0x114 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c3bndtr(&self) -> &C3BNDTR {
        &self.c3bndtr
    }
    ///0x118 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c3sar(&self) -> &C3SAR {
        &self.c3sar
    }
    ///0x11c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c3dar(&self) -> &C3DAR {
        &self.c3dar
    }
    ///0x120 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c3brur(&self) -> &C3BRUR {
        &self.c3brur
    }
    ///0x124 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c3lar(&self) -> &C3LAR {
        &self.c3lar
    }
    ///0x128 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c3tbr(&self) -> &C3TBR {
        &self.c3tbr
    }
    ///0x130 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c3mar(&self) -> &C3MAR {
        &self.c3mar
    }
    ///0x134 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c3mdr(&self) -> &C3MDR {
        &self.c3mdr
    }
    ///0x140 - MDMA channel 4 interrupt/status register
    #[inline(always)]
    pub const fn c4isr(&self) -> &C4ISR {
        &self.c4isr
    }
    ///0x144 - MDMA channel 4 interrupt flag clear register
    #[inline(always)]
    pub const fn c4ifcr(&self) -> &C4IFCR {
        &self.c4ifcr
    }
    ///0x148 - MDMA channel 4 error status register
    #[inline(always)]
    pub const fn c4esr(&self) -> &C4ESR {
        &self.c4esr
    }
    ///0x14c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c4cr(&self) -> &C4CR {
        &self.c4cr
    }
    ///0x150 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c4tcr(&self) -> &C4TCR {
        &self.c4tcr
    }
    ///0x154 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c4bndtr(&self) -> &C4BNDTR {
        &self.c4bndtr
    }
    ///0x158 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c4sar(&self) -> &C4SAR {
        &self.c4sar
    }
    ///0x15c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c4dar(&self) -> &C4DAR {
        &self.c4dar
    }
    ///0x160 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c4brur(&self) -> &C4BRUR {
        &self.c4brur
    }
    ///0x164 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c4lar(&self) -> &C4LAR {
        &self.c4lar
    }
    ///0x168 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c4tbr(&self) -> &C4TBR {
        &self.c4tbr
    }
    ///0x170 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c4mar(&self) -> &C4MAR {
        &self.c4mar
    }
    ///0x174 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c4mdr(&self) -> &C4MDR {
        &self.c4mdr
    }
    ///0x180 - MDMA channel 5 interrupt/status register
    #[inline(always)]
    pub const fn c5isr(&self) -> &C5ISR {
        &self.c5isr
    }
    ///0x184 - MDMA channel 5 interrupt flag clear register
    #[inline(always)]
    pub const fn c5ifcr(&self) -> &C5IFCR {
        &self.c5ifcr
    }
    ///0x188 - MDMA channel 5 error status register
    #[inline(always)]
    pub const fn c5esr(&self) -> &C5ESR {
        &self.c5esr
    }
    ///0x18c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c5cr(&self) -> &C5CR {
        &self.c5cr
    }
    ///0x190 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c5tcr(&self) -> &C5TCR {
        &self.c5tcr
    }
    ///0x194 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c5bndtr(&self) -> &C5BNDTR {
        &self.c5bndtr
    }
    ///0x198 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c5sar(&self) -> &C5SAR {
        &self.c5sar
    }
    ///0x19c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c5dar(&self) -> &C5DAR {
        &self.c5dar
    }
    ///0x1a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c5brur(&self) -> &C5BRUR {
        &self.c5brur
    }
    ///0x1a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c5lar(&self) -> &C5LAR {
        &self.c5lar
    }
    ///0x1a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c5tbr(&self) -> &C5TBR {
        &self.c5tbr
    }
    ///0x1b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c5mar(&self) -> &C5MAR {
        &self.c5mar
    }
    ///0x1b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c5mdr(&self) -> &C5MDR {
        &self.c5mdr
    }
    ///0x1c0 - MDMA channel 6 interrupt/status register
    #[inline(always)]
    pub const fn c6isr(&self) -> &C6ISR {
        &self.c6isr
    }
    ///0x1c4 - MDMA channel 6 interrupt flag clear register
    #[inline(always)]
    pub const fn c6ifcr(&self) -> &C6IFCR {
        &self.c6ifcr
    }
    ///0x1c8 - MDMA channel 6 error status register
    #[inline(always)]
    pub const fn c6esr(&self) -> &C6ESR {
        &self.c6esr
    }
    ///0x1cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c6cr(&self) -> &C6CR {
        &self.c6cr
    }
    ///0x1d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c6tcr(&self) -> &C6TCR {
        &self.c6tcr
    }
    ///0x1d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c6bndtr(&self) -> &C6BNDTR {
        &self.c6bndtr
    }
    ///0x1d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c6sar(&self) -> &C6SAR {
        &self.c6sar
    }
    ///0x1dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c6dar(&self) -> &C6DAR {
        &self.c6dar
    }
    ///0x1e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c6brur(&self) -> &C6BRUR {
        &self.c6brur
    }
    ///0x1e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c6lar(&self) -> &C6LAR {
        &self.c6lar
    }
    ///0x1e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c6tbr(&self) -> &C6TBR {
        &self.c6tbr
    }
    ///0x1f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c6mar(&self) -> &C6MAR {
        &self.c6mar
    }
    ///0x1f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c6mdr(&self) -> &C6MDR {
        &self.c6mdr
    }
    ///0x200 - MDMA channel 7 interrupt/status register
    #[inline(always)]
    pub const fn c7isr(&self) -> &C7ISR {
        &self.c7isr
    }
    ///0x204 - MDMA channel 7 interrupt flag clear register
    #[inline(always)]
    pub const fn c7ifcr(&self) -> &C7IFCR {
        &self.c7ifcr
    }
    ///0x208 - MDMA channel 7 error status register
    #[inline(always)]
    pub const fn c7esr(&self) -> &C7ESR {
        &self.c7esr
    }
    ///0x20c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c7cr(&self) -> &C7CR {
        &self.c7cr
    }
    ///0x210 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c7tcr(&self) -> &C7TCR {
        &self.c7tcr
    }
    ///0x214 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c7bndtr(&self) -> &C7BNDTR {
        &self.c7bndtr
    }
    ///0x218 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c7sar(&self) -> &C7SAR {
        &self.c7sar
    }
    ///0x21c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c7dar(&self) -> &C7DAR {
        &self.c7dar
    }
    ///0x220 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c7brur(&self) -> &C7BRUR {
        &self.c7brur
    }
    ///0x224 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c7lar(&self) -> &C7LAR {
        &self.c7lar
    }
    ///0x228 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c7tbr(&self) -> &C7TBR {
        &self.c7tbr
    }
    ///0x230 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c7mar(&self) -> &C7MAR {
        &self.c7mar
    }
    ///0x234 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c7mdr(&self) -> &C7MDR {
        &self.c7mdr
    }
    ///0x240 - MDMA channel 8 interrupt/status register
    #[inline(always)]
    pub const fn c8isr(&self) -> &C8ISR {
        &self.c8isr
    }
    ///0x244 - MDMA channel 8 interrupt flag clear register
    #[inline(always)]
    pub const fn c8ifcr(&self) -> &C8IFCR {
        &self.c8ifcr
    }
    ///0x248 - MDMA channel 8 error status register
    #[inline(always)]
    pub const fn c8esr(&self) -> &C8ESR {
        &self.c8esr
    }
    ///0x24c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c8cr(&self) -> &C8CR {
        &self.c8cr
    }
    ///0x250 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c8tcr(&self) -> &C8TCR {
        &self.c8tcr
    }
    ///0x254 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c8bndtr(&self) -> &C8BNDTR {
        &self.c8bndtr
    }
    ///0x258 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c8sar(&self) -> &C8SAR {
        &self.c8sar
    }
    ///0x25c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c8dar(&self) -> &C8DAR {
        &self.c8dar
    }
    ///0x260 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c8brur(&self) -> &C8BRUR {
        &self.c8brur
    }
    ///0x264 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c8lar(&self) -> &C8LAR {
        &self.c8lar
    }
    ///0x268 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c8tbr(&self) -> &C8TBR {
        &self.c8tbr
    }
    ///0x270 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c8mar(&self) -> &C8MAR {
        &self.c8mar
    }
    ///0x274 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c8mdr(&self) -> &C8MDR {
        &self.c8mdr
    }
    ///0x280 - MDMA channel 9 interrupt/status register
    #[inline(always)]
    pub const fn c9isr(&self) -> &C9ISR {
        &self.c9isr
    }
    ///0x284 - MDMA channel 9 interrupt flag clear register
    #[inline(always)]
    pub const fn c9ifcr(&self) -> &C9IFCR {
        &self.c9ifcr
    }
    ///0x288 - MDMA channel 9 error status register
    #[inline(always)]
    pub const fn c9esr(&self) -> &C9ESR {
        &self.c9esr
    }
    ///0x28c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c9cr(&self) -> &C9CR {
        &self.c9cr
    }
    ///0x290 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c9tcr(&self) -> &C9TCR {
        &self.c9tcr
    }
    ///0x294 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c9bndtr(&self) -> &C9BNDTR {
        &self.c9bndtr
    }
    ///0x298 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c9sar(&self) -> &C9SAR {
        &self.c9sar
    }
    ///0x29c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c9dar(&self) -> &C9DAR {
        &self.c9dar
    }
    ///0x2a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c9brur(&self) -> &C9BRUR {
        &self.c9brur
    }
    ///0x2a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c9lar(&self) -> &C9LAR {
        &self.c9lar
    }
    ///0x2a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c9tbr(&self) -> &C9TBR {
        &self.c9tbr
    }
    ///0x2b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c9mar(&self) -> &C9MAR {
        &self.c9mar
    }
    ///0x2b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c9mdr(&self) -> &C9MDR {
        &self.c9mdr
    }
    ///0x2c0 - MDMA channel 10 interrupt/status register
    #[inline(always)]
    pub const fn c10isr(&self) -> &C10ISR {
        &self.c10isr
    }
    ///0x2c4 - MDMA channel 10 interrupt flag clear register
    #[inline(always)]
    pub const fn c10ifcr(&self) -> &C10IFCR {
        &self.c10ifcr
    }
    ///0x2c8 - MDMA channel 10 error status register
    #[inline(always)]
    pub const fn c10esr(&self) -> &C10ESR {
        &self.c10esr
    }
    ///0x2cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c10cr(&self) -> &C10CR {
        &self.c10cr
    }
    ///0x2d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c10tcr(&self) -> &C10TCR {
        &self.c10tcr
    }
    ///0x2d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c10bndtr(&self) -> &C10BNDTR {
        &self.c10bndtr
    }
    ///0x2d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c10sar(&self) -> &C10SAR {
        &self.c10sar
    }
    ///0x2dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c10dar(&self) -> &C10DAR {
        &self.c10dar
    }
    ///0x2e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c10brur(&self) -> &C10BRUR {
        &self.c10brur
    }
    ///0x2e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c10lar(&self) -> &C10LAR {
        &self.c10lar
    }
    ///0x2e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c10tbr(&self) -> &C10TBR {
        &self.c10tbr
    }
    ///0x2f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c10mar(&self) -> &C10MAR {
        &self.c10mar
    }
    ///0x2f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c10mdr(&self) -> &C10MDR {
        &self.c10mdr
    }
    ///0x300 - MDMA channel 11 interrupt/status register
    #[inline(always)]
    pub const fn c11isr(&self) -> &C11ISR {
        &self.c11isr
    }
    ///0x304 - MDMA channel 11 interrupt flag clear register
    #[inline(always)]
    pub const fn c11ifcr(&self) -> &C11IFCR {
        &self.c11ifcr
    }
    ///0x308 - MDMA channel 11 error status register
    #[inline(always)]
    pub const fn c11esr(&self) -> &C11ESR {
        &self.c11esr
    }
    ///0x30c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c11cr(&self) -> &C11CR {
        &self.c11cr
    }
    ///0x310 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c11tcr(&self) -> &C11TCR {
        &self.c11tcr
    }
    ///0x314 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c11bndtr(&self) -> &C11BNDTR {
        &self.c11bndtr
    }
    ///0x318 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c11sar(&self) -> &C11SAR {
        &self.c11sar
    }
    ///0x31c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c11dar(&self) -> &C11DAR {
        &self.c11dar
    }
    ///0x320 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c11brur(&self) -> &C11BRUR {
        &self.c11brur
    }
    ///0x324 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c11lar(&self) -> &C11LAR {
        &self.c11lar
    }
    ///0x328 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c11tbr(&self) -> &C11TBR {
        &self.c11tbr
    }
    ///0x330 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c11mar(&self) -> &C11MAR {
        &self.c11mar
    }
    ///0x334 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c11mdr(&self) -> &C11MDR {
        &self.c11mdr
    }
    ///0x340 - MDMA channel 12 interrupt/status register
    #[inline(always)]
    pub const fn c12isr(&self) -> &C12ISR {
        &self.c12isr
    }
    ///0x344 - MDMA channel 12 interrupt flag clear register
    #[inline(always)]
    pub const fn c12ifcr(&self) -> &C12IFCR {
        &self.c12ifcr
    }
    ///0x348 - MDMA channel 12 error status register
    #[inline(always)]
    pub const fn c12esr(&self) -> &C12ESR {
        &self.c12esr
    }
    ///0x34c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c12cr(&self) -> &C12CR {
        &self.c12cr
    }
    ///0x350 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c12tcr(&self) -> &C12TCR {
        &self.c12tcr
    }
    ///0x354 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c12bndtr(&self) -> &C12BNDTR {
        &self.c12bndtr
    }
    ///0x358 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c12sar(&self) -> &C12SAR {
        &self.c12sar
    }
    ///0x35c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c12dar(&self) -> &C12DAR {
        &self.c12dar
    }
    ///0x360 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c12brur(&self) -> &C12BRUR {
        &self.c12brur
    }
    ///0x364 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c12lar(&self) -> &C12LAR {
        &self.c12lar
    }
    ///0x368 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c12tbr(&self) -> &C12TBR {
        &self.c12tbr
    }
    ///0x370 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c12mar(&self) -> &C12MAR {
        &self.c12mar
    }
    ///0x374 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c12mdr(&self) -> &C12MDR {
        &self.c12mdr
    }
    ///0x380 - MDMA channel 13 interrupt/status register
    #[inline(always)]
    pub const fn c13isr(&self) -> &C13ISR {
        &self.c13isr
    }
    ///0x384 - MDMA channel 13 interrupt flag clear register
    #[inline(always)]
    pub const fn c13ifcr(&self) -> &C13IFCR {
        &self.c13ifcr
    }
    ///0x388 - MDMA channel 13 error status register
    #[inline(always)]
    pub const fn c13esr(&self) -> &C13ESR {
        &self.c13esr
    }
    ///0x38c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c13cr(&self) -> &C13CR {
        &self.c13cr
    }
    ///0x390 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c13tcr(&self) -> &C13TCR {
        &self.c13tcr
    }
    ///0x394 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c13bndtr(&self) -> &C13BNDTR {
        &self.c13bndtr
    }
    ///0x398 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c13sar(&self) -> &C13SAR {
        &self.c13sar
    }
    ///0x39c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c13dar(&self) -> &C13DAR {
        &self.c13dar
    }
    ///0x3a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c13brur(&self) -> &C13BRUR {
        &self.c13brur
    }
    ///0x3a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c13lar(&self) -> &C13LAR {
        &self.c13lar
    }
    ///0x3a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c13tbr(&self) -> &C13TBR {
        &self.c13tbr
    }
    ///0x3b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c13mar(&self) -> &C13MAR {
        &self.c13mar
    }
    ///0x3b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c13mdr(&self) -> &C13MDR {
        &self.c13mdr
    }
    ///0x3c0 - MDMA channel 14 interrupt/status register
    #[inline(always)]
    pub const fn c14isr(&self) -> &C14ISR {
        &self.c14isr
    }
    ///0x3c4 - MDMA channel 14 interrupt flag clear register
    #[inline(always)]
    pub const fn c14ifcr(&self) -> &C14IFCR {
        &self.c14ifcr
    }
    ///0x3c8 - MDMA channel 14 error status register
    #[inline(always)]
    pub const fn c14esr(&self) -> &C14ESR {
        &self.c14esr
    }
    ///0x3cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c14cr(&self) -> &C14CR {
        &self.c14cr
    }
    ///0x3d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c14tcr(&self) -> &C14TCR {
        &self.c14tcr
    }
    ///0x3d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c14bndtr(&self) -> &C14BNDTR {
        &self.c14bndtr
    }
    ///0x3d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c14sar(&self) -> &C14SAR {
        &self.c14sar
    }
    ///0x3dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c14dar(&self) -> &C14DAR {
        &self.c14dar
    }
    ///0x3e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c14brur(&self) -> &C14BRUR {
        &self.c14brur
    }
    ///0x3e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c14lar(&self) -> &C14LAR {
        &self.c14lar
    }
    ///0x3e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c14tbr(&self) -> &C14TBR {
        &self.c14tbr
    }
    ///0x3f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c14mar(&self) -> &C14MAR {
        &self.c14mar
    }
    ///0x3f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c14mdr(&self) -> &C14MDR {
        &self.c14mdr
    }
    ///0x400 - MDMA channel 15 interrupt/status register
    #[inline(always)]
    pub const fn c15isr(&self) -> &C15ISR {
        &self.c15isr
    }
    ///0x404 - MDMA channel 15 interrupt flag clear register
    #[inline(always)]
    pub const fn c15ifcr(&self) -> &C15IFCR {
        &self.c15ifcr
    }
    ///0x408 - MDMA channel 15 error status register
    #[inline(always)]
    pub const fn c15esr(&self) -> &C15ESR {
        &self.c15esr
    }
    ///0x40c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c15cr(&self) -> &C15CR {
        &self.c15cr
    }
    ///0x410 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c15tcr(&self) -> &C15TCR {
        &self.c15tcr
    }
    ///0x414 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c15bndtr(&self) -> &C15BNDTR {
        &self.c15bndtr
    }
    ///0x418 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c15sar(&self) -> &C15SAR {
        &self.c15sar
    }
    ///0x41c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c15dar(&self) -> &C15DAR {
        &self.c15dar
    }
    ///0x420 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c15brur(&self) -> &C15BRUR {
        &self.c15brur
    }
    ///0x424 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c15lar(&self) -> &C15LAR {
        &self.c15lar
    }
    ///0x428 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c15tbr(&self) -> &C15TBR {
        &self.c15tbr
    }
    ///0x430 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c15mar(&self) -> &C15MAR {
        &self.c15mar
    }
    ///0x434 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c15mdr(&self) -> &C15MDR {
        &self.c15mdr
    }
    ///0x440 - MDMA channel 16 interrupt/status register
    #[inline(always)]
    pub const fn c16isr(&self) -> &C16ISR {
        &self.c16isr
    }
    ///0x444 - MDMA channel 16 interrupt flag clear register
    #[inline(always)]
    pub const fn c16ifcr(&self) -> &C16IFCR {
        &self.c16ifcr
    }
    ///0x448 - MDMA channel 16 error status register
    #[inline(always)]
    pub const fn c16esr(&self) -> &C16ESR {
        &self.c16esr
    }
    ///0x44c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c16cr(&self) -> &C16CR {
        &self.c16cr
    }
    ///0x450 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c16tcr(&self) -> &C16TCR {
        &self.c16tcr
    }
    ///0x454 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c16bndtr(&self) -> &C16BNDTR {
        &self.c16bndtr
    }
    ///0x458 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c16sar(&self) -> &C16SAR {
        &self.c16sar
    }
    ///0x45c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c16dar(&self) -> &C16DAR {
        &self.c16dar
    }
    ///0x460 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c16brur(&self) -> &C16BRUR {
        &self.c16brur
    }
    ///0x464 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c16lar(&self) -> &C16LAR {
        &self.c16lar
    }
    ///0x468 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c16tbr(&self) -> &C16TBR {
        &self.c16tbr
    }
    ///0x470 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c16mar(&self) -> &C16MAR {
        &self.c16mar
    }
    ///0x474 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c16mdr(&self) -> &C16MDR {
        &self.c16mdr
    }
    ///0x480 - MDMA channel 17 interrupt/status register
    #[inline(always)]
    pub const fn c17isr(&self) -> &C17ISR {
        &self.c17isr
    }
    ///0x484 - MDMA channel 17 interrupt flag clear register
    #[inline(always)]
    pub const fn c17ifcr(&self) -> &C17IFCR {
        &self.c17ifcr
    }
    ///0x488 - MDMA channel 17 error status register
    #[inline(always)]
    pub const fn c17esr(&self) -> &C17ESR {
        &self.c17esr
    }
    ///0x48c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c17cr(&self) -> &C17CR {
        &self.c17cr
    }
    ///0x490 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c17tcr(&self) -> &C17TCR {
        &self.c17tcr
    }
    ///0x494 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c17bndtr(&self) -> &C17BNDTR {
        &self.c17bndtr
    }
    ///0x498 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c17sar(&self) -> &C17SAR {
        &self.c17sar
    }
    ///0x49c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c17dar(&self) -> &C17DAR {
        &self.c17dar
    }
    ///0x4a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c17brur(&self) -> &C17BRUR {
        &self.c17brur
    }
    ///0x4a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c17lar(&self) -> &C17LAR {
        &self.c17lar
    }
    ///0x4a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c17tbr(&self) -> &C17TBR {
        &self.c17tbr
    }
    ///0x4b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c17mar(&self) -> &C17MAR {
        &self.c17mar
    }
    ///0x4b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c17mdr(&self) -> &C17MDR {
        &self.c17mdr
    }
    ///0x4c0 - MDMA channel 18 interrupt/status register
    #[inline(always)]
    pub const fn c18isr(&self) -> &C18ISR {
        &self.c18isr
    }
    ///0x4c4 - MDMA channel 18 interrupt flag clear register
    #[inline(always)]
    pub const fn c18ifcr(&self) -> &C18IFCR {
        &self.c18ifcr
    }
    ///0x4c8 - MDMA channel 18 error status register
    #[inline(always)]
    pub const fn c18esr(&self) -> &C18ESR {
        &self.c18esr
    }
    ///0x4cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c18cr(&self) -> &C18CR {
        &self.c18cr
    }
    ///0x4d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c18tcr(&self) -> &C18TCR {
        &self.c18tcr
    }
    ///0x4d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c18bndtr(&self) -> &C18BNDTR {
        &self.c18bndtr
    }
    ///0x4d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c18sar(&self) -> &C18SAR {
        &self.c18sar
    }
    ///0x4dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c18dar(&self) -> &C18DAR {
        &self.c18dar
    }
    ///0x4e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c18brur(&self) -> &C18BRUR {
        &self.c18brur
    }
    ///0x4e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c18lar(&self) -> &C18LAR {
        &self.c18lar
    }
    ///0x4e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c18tbr(&self) -> &C18TBR {
        &self.c18tbr
    }
    ///0x4f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c18mar(&self) -> &C18MAR {
        &self.c18mar
    }
    ///0x4f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c18mdr(&self) -> &C18MDR {
        &self.c18mdr
    }
    ///0x500 - MDMA channel 19 interrupt/status register
    #[inline(always)]
    pub const fn c19isr(&self) -> &C19ISR {
        &self.c19isr
    }
    ///0x504 - MDMA channel 19 interrupt flag clear register
    #[inline(always)]
    pub const fn c19ifcr(&self) -> &C19IFCR {
        &self.c19ifcr
    }
    ///0x508 - MDMA channel 19 error status register
    #[inline(always)]
    pub const fn c19esr(&self) -> &C19ESR {
        &self.c19esr
    }
    ///0x50c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c19cr(&self) -> &C19CR {
        &self.c19cr
    }
    ///0x510 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c19tcr(&self) -> &C19TCR {
        &self.c19tcr
    }
    ///0x514 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c19bndtr(&self) -> &C19BNDTR {
        &self.c19bndtr
    }
    ///0x518 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c19sar(&self) -> &C19SAR {
        &self.c19sar
    }
    ///0x51c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c19dar(&self) -> &C19DAR {
        &self.c19dar
    }
    ///0x520 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c19brur(&self) -> &C19BRUR {
        &self.c19brur
    }
    ///0x524 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c19lar(&self) -> &C19LAR {
        &self.c19lar
    }
    ///0x528 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c19tbr(&self) -> &C19TBR {
        &self.c19tbr
    }
    ///0x530 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c19mar(&self) -> &C19MAR {
        &self.c19mar
    }
    ///0x534 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c19mdr(&self) -> &C19MDR {
        &self.c19mdr
    }
    ///0x540 - MDMA channel 20 interrupt/status register
    #[inline(always)]
    pub const fn c20isr(&self) -> &C20ISR {
        &self.c20isr
    }
    ///0x544 - MDMA channel 20 interrupt flag clear register
    #[inline(always)]
    pub const fn c20ifcr(&self) -> &C20IFCR {
        &self.c20ifcr
    }
    ///0x548 - MDMA channel 20 error status register
    #[inline(always)]
    pub const fn c20esr(&self) -> &C20ESR {
        &self.c20esr
    }
    ///0x54c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c20cr(&self) -> &C20CR {
        &self.c20cr
    }
    ///0x550 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c20tcr(&self) -> &C20TCR {
        &self.c20tcr
    }
    ///0x554 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c20bndtr(&self) -> &C20BNDTR {
        &self.c20bndtr
    }
    ///0x558 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c20sar(&self) -> &C20SAR {
        &self.c20sar
    }
    ///0x55c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c20dar(&self) -> &C20DAR {
        &self.c20dar
    }
    ///0x560 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c20brur(&self) -> &C20BRUR {
        &self.c20brur
    }
    ///0x564 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c20lar(&self) -> &C20LAR {
        &self.c20lar
    }
    ///0x568 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c20tbr(&self) -> &C20TBR {
        &self.c20tbr
    }
    ///0x570 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c20mar(&self) -> &C20MAR {
        &self.c20mar
    }
    ///0x574 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c20mdr(&self) -> &C20MDR {
        &self.c20mdr
    }
    ///0x580 - MDMA channel 21 interrupt/status register
    #[inline(always)]
    pub const fn c21isr(&self) -> &C21ISR {
        &self.c21isr
    }
    ///0x584 - MDMA channel 21 interrupt flag clear register
    #[inline(always)]
    pub const fn c21ifcr(&self) -> &C21IFCR {
        &self.c21ifcr
    }
    ///0x588 - MDMA channel 21 error status register
    #[inline(always)]
    pub const fn c21esr(&self) -> &C21ESR {
        &self.c21esr
    }
    ///0x58c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c21cr(&self) -> &C21CR {
        &self.c21cr
    }
    ///0x590 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c21tcr(&self) -> &C21TCR {
        &self.c21tcr
    }
    ///0x594 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c21bndtr(&self) -> &C21BNDTR {
        &self.c21bndtr
    }
    ///0x598 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c21sar(&self) -> &C21SAR {
        &self.c21sar
    }
    ///0x59c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c21dar(&self) -> &C21DAR {
        &self.c21dar
    }
    ///0x5a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c21brur(&self) -> &C21BRUR {
        &self.c21brur
    }
    ///0x5a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c21lar(&self) -> &C21LAR {
        &self.c21lar
    }
    ///0x5a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c21tbr(&self) -> &C21TBR {
        &self.c21tbr
    }
    ///0x5b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c21mar(&self) -> &C21MAR {
        &self.c21mar
    }
    ///0x5b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c21mdr(&self) -> &C21MDR {
        &self.c21mdr
    }
    ///0x5c0 - MDMA channel 22 interrupt/status register
    #[inline(always)]
    pub const fn c22isr(&self) -> &C22ISR {
        &self.c22isr
    }
    ///0x5c4 - MDMA channel 22 interrupt flag clear register
    #[inline(always)]
    pub const fn c22ifcr(&self) -> &C22IFCR {
        &self.c22ifcr
    }
    ///0x5c8 - MDMA channel 22 error status register
    #[inline(always)]
    pub const fn c22esr(&self) -> &C22ESR {
        &self.c22esr
    }
    ///0x5cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c22cr(&self) -> &C22CR {
        &self.c22cr
    }
    ///0x5d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c22tcr(&self) -> &C22TCR {
        &self.c22tcr
    }
    ///0x5d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c22bndtr(&self) -> &C22BNDTR {
        &self.c22bndtr
    }
    ///0x5d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c22sar(&self) -> &C22SAR {
        &self.c22sar
    }
    ///0x5dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c22dar(&self) -> &C22DAR {
        &self.c22dar
    }
    ///0x5e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c22brur(&self) -> &C22BRUR {
        &self.c22brur
    }
    ///0x5e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c22lar(&self) -> &C22LAR {
        &self.c22lar
    }
    ///0x5e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c22tbr(&self) -> &C22TBR {
        &self.c22tbr
    }
    ///0x5f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c22mar(&self) -> &C22MAR {
        &self.c22mar
    }
    ///0x5f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c22mdr(&self) -> &C22MDR {
        &self.c22mdr
    }
    ///0x600 - MDMA channel 23 interrupt/status register
    #[inline(always)]
    pub const fn c23isr(&self) -> &C23ISR {
        &self.c23isr
    }
    ///0x604 - MDMA channel 23 interrupt flag clear register
    #[inline(always)]
    pub const fn c23ifcr(&self) -> &C23IFCR {
        &self.c23ifcr
    }
    ///0x608 - MDMA channel 23 error status register
    #[inline(always)]
    pub const fn c23esr(&self) -> &C23ESR {
        &self.c23esr
    }
    ///0x60c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c23cr(&self) -> &C23CR {
        &self.c23cr
    }
    ///0x610 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c23tcr(&self) -> &C23TCR {
        &self.c23tcr
    }
    ///0x614 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c23bndtr(&self) -> &C23BNDTR {
        &self.c23bndtr
    }
    ///0x618 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c23sar(&self) -> &C23SAR {
        &self.c23sar
    }
    ///0x61c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c23dar(&self) -> &C23DAR {
        &self.c23dar
    }
    ///0x620 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c23brur(&self) -> &C23BRUR {
        &self.c23brur
    }
    ///0x624 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c23lar(&self) -> &C23LAR {
        &self.c23lar
    }
    ///0x628 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c23tbr(&self) -> &C23TBR {
        &self.c23tbr
    }
    ///0x630 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c23mar(&self) -> &C23MAR {
        &self.c23mar
    }
    ///0x634 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c23mdr(&self) -> &C23MDR {
        &self.c23mdr
    }
    ///0x640 - MDMA channel 24 interrupt/status register
    #[inline(always)]
    pub const fn c24isr(&self) -> &C24ISR {
        &self.c24isr
    }
    ///0x644 - MDMA channel 24 interrupt flag clear register
    #[inline(always)]
    pub const fn c24ifcr(&self) -> &C24IFCR {
        &self.c24ifcr
    }
    ///0x648 - MDMA channel 24 error status register
    #[inline(always)]
    pub const fn c24esr(&self) -> &C24ESR {
        &self.c24esr
    }
    ///0x64c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c24cr(&self) -> &C24CR {
        &self.c24cr
    }
    ///0x650 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c24tcr(&self) -> &C24TCR {
        &self.c24tcr
    }
    ///0x654 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c24bndtr(&self) -> &C24BNDTR {
        &self.c24bndtr
    }
    ///0x658 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c24sar(&self) -> &C24SAR {
        &self.c24sar
    }
    ///0x65c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c24dar(&self) -> &C24DAR {
        &self.c24dar
    }
    ///0x660 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c24brur(&self) -> &C24BRUR {
        &self.c24brur
    }
    ///0x664 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c24lar(&self) -> &C24LAR {
        &self.c24lar
    }
    ///0x668 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c24tbr(&self) -> &C24TBR {
        &self.c24tbr
    }
    ///0x670 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c24mar(&self) -> &C24MAR {
        &self.c24mar
    }
    ///0x674 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c24mdr(&self) -> &C24MDR {
        &self.c24mdr
    }
    ///0x680 - MDMA channel 25 interrupt/status register
    #[inline(always)]
    pub const fn c25isr(&self) -> &C25ISR {
        &self.c25isr
    }
    ///0x684 - MDMA channel 25 interrupt flag clear register
    #[inline(always)]
    pub const fn c25ifcr(&self) -> &C25IFCR {
        &self.c25ifcr
    }
    ///0x688 - MDMA channel 25 error status register
    #[inline(always)]
    pub const fn c25esr(&self) -> &C25ESR {
        &self.c25esr
    }
    ///0x68c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c25cr(&self) -> &C25CR {
        &self.c25cr
    }
    ///0x690 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c25tcr(&self) -> &C25TCR {
        &self.c25tcr
    }
    ///0x694 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c25bndtr(&self) -> &C25BNDTR {
        &self.c25bndtr
    }
    ///0x698 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c25sar(&self) -> &C25SAR {
        &self.c25sar
    }
    ///0x69c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c25dar(&self) -> &C25DAR {
        &self.c25dar
    }
    ///0x6a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c25brur(&self) -> &C25BRUR {
        &self.c25brur
    }
    ///0x6a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c25lar(&self) -> &C25LAR {
        &self.c25lar
    }
    ///0x6a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c25tbr(&self) -> &C25TBR {
        &self.c25tbr
    }
    ///0x6b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c25mar(&self) -> &C25MAR {
        &self.c25mar
    }
    ///0x6b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c25mdr(&self) -> &C25MDR {
        &self.c25mdr
    }
    ///0x6c0 - MDMA channel 26 interrupt/status register
    #[inline(always)]
    pub const fn c26isr(&self) -> &C26ISR {
        &self.c26isr
    }
    ///0x6c4 - MDMA channel 26 interrupt flag clear register
    #[inline(always)]
    pub const fn c26ifcr(&self) -> &C26IFCR {
        &self.c26ifcr
    }
    ///0x6c8 - MDMA channel 26 error status register
    #[inline(always)]
    pub const fn c26esr(&self) -> &C26ESR {
        &self.c26esr
    }
    ///0x6cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c26cr(&self) -> &C26CR {
        &self.c26cr
    }
    ///0x6d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c26tcr(&self) -> &C26TCR {
        &self.c26tcr
    }
    ///0x6d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c26bndtr(&self) -> &C26BNDTR {
        &self.c26bndtr
    }
    ///0x6d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c26sar(&self) -> &C26SAR {
        &self.c26sar
    }
    ///0x6dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c26dar(&self) -> &C26DAR {
        &self.c26dar
    }
    ///0x6e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c26brur(&self) -> &C26BRUR {
        &self.c26brur
    }
    ///0x6e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c26lar(&self) -> &C26LAR {
        &self.c26lar
    }
    ///0x6e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c26tbr(&self) -> &C26TBR {
        &self.c26tbr
    }
    ///0x6f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c26mar(&self) -> &C26MAR {
        &self.c26mar
    }
    ///0x6f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c26mdr(&self) -> &C26MDR {
        &self.c26mdr
    }
    ///0x700 - MDMA channel 27 interrupt/status register
    #[inline(always)]
    pub const fn c27isr(&self) -> &C27ISR {
        &self.c27isr
    }
    ///0x704 - MDMA channel 27 interrupt flag clear register
    #[inline(always)]
    pub const fn c27ifcr(&self) -> &C27IFCR {
        &self.c27ifcr
    }
    ///0x708 - MDMA channel 27 error status register
    #[inline(always)]
    pub const fn c27esr(&self) -> &C27ESR {
        &self.c27esr
    }
    ///0x70c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c27cr(&self) -> &C27CR {
        &self.c27cr
    }
    ///0x710 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c27tcr(&self) -> &C27TCR {
        &self.c27tcr
    }
    ///0x714 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c27bndtr(&self) -> &C27BNDTR {
        &self.c27bndtr
    }
    ///0x718 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c27sar(&self) -> &C27SAR {
        &self.c27sar
    }
    ///0x71c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c27dar(&self) -> &C27DAR {
        &self.c27dar
    }
    ///0x720 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c27brur(&self) -> &C27BRUR {
        &self.c27brur
    }
    ///0x724 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c27lar(&self) -> &C27LAR {
        &self.c27lar
    }
    ///0x728 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c27tbr(&self) -> &C27TBR {
        &self.c27tbr
    }
    ///0x730 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c27mar(&self) -> &C27MAR {
        &self.c27mar
    }
    ///0x734 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c27mdr(&self) -> &C27MDR {
        &self.c27mdr
    }
    ///0x740 - MDMA channel 28 interrupt/status register
    #[inline(always)]
    pub const fn c28isr(&self) -> &C28ISR {
        &self.c28isr
    }
    ///0x744 - MDMA channel 28 interrupt flag clear register
    #[inline(always)]
    pub const fn c28ifcr(&self) -> &C28IFCR {
        &self.c28ifcr
    }
    ///0x748 - MDMA channel 28 error status register
    #[inline(always)]
    pub const fn c28esr(&self) -> &C28ESR {
        &self.c28esr
    }
    ///0x74c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c28cr(&self) -> &C28CR {
        &self.c28cr
    }
    ///0x750 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c28tcr(&self) -> &C28TCR {
        &self.c28tcr
    }
    ///0x754 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c28bndtr(&self) -> &C28BNDTR {
        &self.c28bndtr
    }
    ///0x758 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c28sar(&self) -> &C28SAR {
        &self.c28sar
    }
    ///0x75c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c28dar(&self) -> &C28DAR {
        &self.c28dar
    }
    ///0x760 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c28brur(&self) -> &C28BRUR {
        &self.c28brur
    }
    ///0x764 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c28lar(&self) -> &C28LAR {
        &self.c28lar
    }
    ///0x768 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c28tbr(&self) -> &C28TBR {
        &self.c28tbr
    }
    ///0x770 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c28mar(&self) -> &C28MAR {
        &self.c28mar
    }
    ///0x774 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c28mdr(&self) -> &C28MDR {
        &self.c28mdr
    }
    ///0x780 - MDMA channel 29 interrupt/status register
    #[inline(always)]
    pub const fn c29isr(&self) -> &C29ISR {
        &self.c29isr
    }
    ///0x784 - MDMA channel 29 interrupt flag clear register
    #[inline(always)]
    pub const fn c29ifcr(&self) -> &C29IFCR {
        &self.c29ifcr
    }
    ///0x788 - MDMA channel 29 error status register
    #[inline(always)]
    pub const fn c29esr(&self) -> &C29ESR {
        &self.c29esr
    }
    ///0x78c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c29cr(&self) -> &C29CR {
        &self.c29cr
    }
    ///0x790 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c29tcr(&self) -> &C29TCR {
        &self.c29tcr
    }
    ///0x794 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c29bndtr(&self) -> &C29BNDTR {
        &self.c29bndtr
    }
    ///0x798 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c29sar(&self) -> &C29SAR {
        &self.c29sar
    }
    ///0x79c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c29dar(&self) -> &C29DAR {
        &self.c29dar
    }
    ///0x7a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c29brur(&self) -> &C29BRUR {
        &self.c29brur
    }
    ///0x7a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c29lar(&self) -> &C29LAR {
        &self.c29lar
    }
    ///0x7a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c29tbr(&self) -> &C29TBR {
        &self.c29tbr
    }
    ///0x7b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c29mar(&self) -> &C29MAR {
        &self.c29mar
    }
    ///0x7b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c29mdr(&self) -> &C29MDR {
        &self.c29mdr
    }
    ///0x7c0 - MDMA channel 30 interrupt/status register
    #[inline(always)]
    pub const fn c30isr(&self) -> &C30ISR {
        &self.c30isr
    }
    ///0x7c4 - MDMA channel 30 interrupt flag clear register
    #[inline(always)]
    pub const fn c30ifcr(&self) -> &C30IFCR {
        &self.c30ifcr
    }
    ///0x7c8 - MDMA channel 30 error status register
    #[inline(always)]
    pub const fn c30esr(&self) -> &C30ESR {
        &self.c30esr
    }
    ///0x7cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c30cr(&self) -> &C30CR {
        &self.c30cr
    }
    ///0x7d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c30tcr(&self) -> &C30TCR {
        &self.c30tcr
    }
    ///0x7d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c30bndtr(&self) -> &C30BNDTR {
        &self.c30bndtr
    }
    ///0x7d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c30sar(&self) -> &C30SAR {
        &self.c30sar
    }
    ///0x7dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c30dar(&self) -> &C30DAR {
        &self.c30dar
    }
    ///0x7e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c30brur(&self) -> &C30BRUR {
        &self.c30brur
    }
    ///0x7e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c30lar(&self) -> &C30LAR {
        &self.c30lar
    }
    ///0x7e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c30tbr(&self) -> &C30TBR {
        &self.c30tbr
    }
    ///0x7f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c30mar(&self) -> &C30MAR {
        &self.c30mar
    }
    ///0x7f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c30mdr(&self) -> &C30MDR {
        &self.c30mdr
    }
    ///0x800 - MDMA channel 31 interrupt/status register
    #[inline(always)]
    pub const fn c31isr(&self) -> &C31ISR {
        &self.c31isr
    }
    ///0x804 - MDMA channel 31 interrupt flag clear register
    #[inline(always)]
    pub const fn c31ifcr(&self) -> &C31IFCR {
        &self.c31ifcr
    }
    ///0x808 - MDMA channel 31 error status register
    #[inline(always)]
    pub const fn c31esr(&self) -> &C31ESR {
        &self.c31esr
    }
    ///0x80c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn c31cr(&self) -> &C31CR {
        &self.c31cr
    }
    ///0x810 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
    #[inline(always)]
    pub const fn c31tcr(&self) -> &C31TCR {
        &self.c31tcr
    }
    ///0x814 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
    #[inline(always)]
    pub const fn c31bndtr(&self) -> &C31BNDTR {
        &self.c31bndtr
    }
    ///0x818 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
    #[inline(always)]
    pub const fn c31sar(&self) -> &C31SAR {
        &self.c31sar
    }
    ///0x81c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
    #[inline(always)]
    pub const fn c31dar(&self) -> &C31DAR {
        &self.c31dar
    }
    ///0x820 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
    #[inline(always)]
    pub const fn c31brur(&self) -> &C31BRUR {
        &self.c31brur
    }
    ///0x824 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
    #[inline(always)]
    pub const fn c31lar(&self) -> &C31LAR {
        &self.c31lar
    }
    ///0x828 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
    #[inline(always)]
    pub const fn c31tbr(&self) -> &C31TBR {
        &self.c31tbr
    }
    ///0x830 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
    #[inline(always)]
    pub const fn c31mar(&self) -> &C31MAR {
        &self.c31mar
    }
    ///0x834 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
    #[inline(always)]
    pub const fn c31mdr(&self) -> &C31MDR {
        &self.c31mdr
    }
}
/**GISR0 (r) register accessor: MDMA global interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`gisr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:GISR0)

For information about available fields see [`mod@gisr0`] module*/
pub type GISR0 = crate::Reg<gisr0::GISR0rs>;
///MDMA global interrupt/status register
pub mod gisr0;
/**SGISR0 (r) register accessor: MDMA secure global interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`sgisr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:SGISR0)

For information about available fields see [`mod@sgisr0`] module*/
pub type SGISR0 = crate::Reg<sgisr0::SGISR0rs>;
///MDMA secure global interrupt/status register
pub mod sgisr0;
/**C0ISR (r) register accessor: MDMA channel 0 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c0isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0ISR)

For information about available fields see [`mod@c0isr`] module*/
pub type C0ISR = crate::Reg<c0isr::C0ISRrs>;
///MDMA channel 0 interrupt/status register
pub mod c0isr;
/**C0IFCR (w) register accessor: MDMA channel 0 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0IFCR)

For information about available fields see [`mod@c0ifcr`] module*/
pub type C0IFCR = crate::Reg<c0ifcr::C0IFCRrs>;
///MDMA channel 0 interrupt flag clear register
pub mod c0ifcr;
/**C0ESR (r) register accessor: MDMA channel 0 error status register

You can [`read`](crate::Reg::read) this register and get [`c0esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0ESR)

For information about available fields see [`mod@c0esr`] module*/
pub type C0ESR = crate::Reg<c0esr::C0ESRrs>;
///MDMA channel 0 error status register
pub mod c0esr;
/**C0CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0CR)

For information about available fields see [`mod@c0cr`] module*/
pub type C0CR = crate::Reg<c0cr::C0CRrs>;
///This register is used to control the concerned channel.
pub mod c0cr;
/**C0TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c0tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0TCR)

For information about available fields see [`mod@c0tcr`] module*/
pub type C0TCR = crate::Reg<c0tcr::C0TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c0tcr;
/**C0BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c0bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0BNDTR)

For information about available fields see [`mod@c0bndtr`] module*/
pub type C0BNDTR = crate::Reg<c0bndtr::C0BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c0bndtr;
/**C0SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c0sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0SAR)

For information about available fields see [`mod@c0sar`] module*/
pub type C0SAR = crate::Reg<c0sar::C0SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c0sar;
/**C0DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c0dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0DAR)

For information about available fields see [`mod@c0dar`] module*/
pub type C0DAR = crate::Reg<c0dar::C0DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c0dar;
/**C0BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c0brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0BRUR)

For information about available fields see [`mod@c0brur`] module*/
pub type C0BRUR = crate::Reg<c0brur::C0BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c0brur;
/**C0LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c0lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0LAR)

For information about available fields see [`mod@c0lar`] module*/
pub type C0LAR = crate::Reg<c0lar::C0LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c0lar;
/**C0TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c0tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0TBR)

For information about available fields see [`mod@c0tbr`] module*/
pub type C0TBR = crate::Reg<c0tbr::C0TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c0tbr;
/**C0MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c0mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0MAR)

For information about available fields see [`mod@c0mar`] module*/
pub type C0MAR = crate::Reg<c0mar::C0MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c0mar;
/**C0MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c0mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c0mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C0MDR)

For information about available fields see [`mod@c0mdr`] module*/
pub type C0MDR = crate::Reg<c0mdr::C0MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c0mdr;
/**C1ISR (r) register accessor: MDMA channel 1 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1ISR)

For information about available fields see [`mod@c1isr`] module*/
pub type C1ISR = crate::Reg<c1isr::C1ISRrs>;
///MDMA channel 1 interrupt/status register
pub mod c1isr;
/**C1IFCR (w) register accessor: MDMA channel 1 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1IFCR)

For information about available fields see [`mod@c1ifcr`] module*/
pub type C1IFCR = crate::Reg<c1ifcr::C1IFCRrs>;
///MDMA channel 1 interrupt flag clear register
pub mod c1ifcr;
/**C1ESR (r) register accessor: MDMA channel 1 error status register

You can [`read`](crate::Reg::read) this register and get [`c1esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1ESR)

For information about available fields see [`mod@c1esr`] module*/
pub type C1ESR = crate::Reg<c1esr::C1ESRrs>;
///MDMA channel 1 error status register
pub mod c1esr;
/**C1CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1CR)

For information about available fields see [`mod@c1cr`] module*/
pub type C1CR = crate::Reg<c1cr::C1CRrs>;
///This register is used to control the concerned channel.
pub mod c1cr;
/**C1TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c1tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1TCR)

For information about available fields see [`mod@c1tcr`] module*/
pub type C1TCR = crate::Reg<c1tcr::C1TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c1tcr;
/**C1BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c1bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1BNDTR)

For information about available fields see [`mod@c1bndtr`] module*/
pub type C1BNDTR = crate::Reg<c1bndtr::C1BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c1bndtr;
/**C1SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c1sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1SAR)

For information about available fields see [`mod@c1sar`] module*/
pub type C1SAR = crate::Reg<c1sar::C1SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c1sar;
/**C1DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c1dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1DAR)

For information about available fields see [`mod@c1dar`] module*/
pub type C1DAR = crate::Reg<c1dar::C1DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c1dar;
/**C1BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c1brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1BRUR)

For information about available fields see [`mod@c1brur`] module*/
pub type C1BRUR = crate::Reg<c1brur::C1BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c1brur;
/**C1LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c1lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1LAR)

For information about available fields see [`mod@c1lar`] module*/
pub type C1LAR = crate::Reg<c1lar::C1LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c1lar;
/**C1TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c1tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1TBR)

For information about available fields see [`mod@c1tbr`] module*/
pub type C1TBR = crate::Reg<c1tbr::C1TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c1tbr;
/**C1MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c1mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1MAR)

For information about available fields see [`mod@c1mar`] module*/
pub type C1MAR = crate::Reg<c1mar::C1MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c1mar;
/**C1MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c1mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C1MDR)

For information about available fields see [`mod@c1mdr`] module*/
pub type C1MDR = crate::Reg<c1mdr::C1MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c1mdr;
/**C2ISR (r) register accessor: MDMA channel 2 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2ISR)

For information about available fields see [`mod@c2isr`] module*/
pub type C2ISR = crate::Reg<c2isr::C2ISRrs>;
///MDMA channel 2 interrupt/status register
pub mod c2isr;
/**C2IFCR (w) register accessor: MDMA channel 2 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2IFCR)

For information about available fields see [`mod@c2ifcr`] module*/
pub type C2IFCR = crate::Reg<c2ifcr::C2IFCRrs>;
///MDMA channel 2 interrupt flag clear register
pub mod c2ifcr;
/**C2ESR (r) register accessor: MDMA channel 2 error status register

You can [`read`](crate::Reg::read) this register and get [`c2esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2ESR)

For information about available fields see [`mod@c2esr`] module*/
pub type C2ESR = crate::Reg<c2esr::C2ESRrs>;
///MDMA channel 2 error status register
pub mod c2esr;
/**C2CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2CR)

For information about available fields see [`mod@c2cr`] module*/
pub type C2CR = crate::Reg<c2cr::C2CRrs>;
///This register is used to control the concerned channel.
pub mod c2cr;
/**C2TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c2tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2TCR)

For information about available fields see [`mod@c2tcr`] module*/
pub type C2TCR = crate::Reg<c2tcr::C2TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c2tcr;
/**C2BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c2bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2BNDTR)

For information about available fields see [`mod@c2bndtr`] module*/
pub type C2BNDTR = crate::Reg<c2bndtr::C2BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c2bndtr;
/**C2SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c2sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2SAR)

For information about available fields see [`mod@c2sar`] module*/
pub type C2SAR = crate::Reg<c2sar::C2SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c2sar;
/**C2DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c2dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2DAR)

For information about available fields see [`mod@c2dar`] module*/
pub type C2DAR = crate::Reg<c2dar::C2DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c2dar;
/**C2BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c2brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2BRUR)

For information about available fields see [`mod@c2brur`] module*/
pub type C2BRUR = crate::Reg<c2brur::C2BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c2brur;
/**C2LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c2lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2LAR)

For information about available fields see [`mod@c2lar`] module*/
pub type C2LAR = crate::Reg<c2lar::C2LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c2lar;
/**C2TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c2tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2TBR)

For information about available fields see [`mod@c2tbr`] module*/
pub type C2TBR = crate::Reg<c2tbr::C2TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c2tbr;
/**C2MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c2mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2MAR)

For information about available fields see [`mod@c2mar`] module*/
pub type C2MAR = crate::Reg<c2mar::C2MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c2mar;
/**C2MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c2mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c2mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C2MDR)

For information about available fields see [`mod@c2mdr`] module*/
pub type C2MDR = crate::Reg<c2mdr::C2MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c2mdr;
/**C3ISR (r) register accessor: MDMA channel 3 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3ISR)

For information about available fields see [`mod@c3isr`] module*/
pub type C3ISR = crate::Reg<c3isr::C3ISRrs>;
///MDMA channel 3 interrupt/status register
pub mod c3isr;
/**C3IFCR (w) register accessor: MDMA channel 3 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3IFCR)

For information about available fields see [`mod@c3ifcr`] module*/
pub type C3IFCR = crate::Reg<c3ifcr::C3IFCRrs>;
///MDMA channel 3 interrupt flag clear register
pub mod c3ifcr;
/**C3ESR (r) register accessor: MDMA channel 3 error status register

You can [`read`](crate::Reg::read) this register and get [`c3esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3ESR)

For information about available fields see [`mod@c3esr`] module*/
pub type C3ESR = crate::Reg<c3esr::C3ESRrs>;
///MDMA channel 3 error status register
pub mod c3esr;
/**C3CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3CR)

For information about available fields see [`mod@c3cr`] module*/
pub type C3CR = crate::Reg<c3cr::C3CRrs>;
///This register is used to control the concerned channel.
pub mod c3cr;
/**C3TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c3tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3TCR)

For information about available fields see [`mod@c3tcr`] module*/
pub type C3TCR = crate::Reg<c3tcr::C3TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c3tcr;
/**C3BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c3bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3BNDTR)

For information about available fields see [`mod@c3bndtr`] module*/
pub type C3BNDTR = crate::Reg<c3bndtr::C3BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c3bndtr;
/**C3SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c3sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3SAR)

For information about available fields see [`mod@c3sar`] module*/
pub type C3SAR = crate::Reg<c3sar::C3SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c3sar;
/**C3DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c3dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3DAR)

For information about available fields see [`mod@c3dar`] module*/
pub type C3DAR = crate::Reg<c3dar::C3DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c3dar;
/**C3BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c3brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3BRUR)

For information about available fields see [`mod@c3brur`] module*/
pub type C3BRUR = crate::Reg<c3brur::C3BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c3brur;
/**C3LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c3lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3LAR)

For information about available fields see [`mod@c3lar`] module*/
pub type C3LAR = crate::Reg<c3lar::C3LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c3lar;
/**C3TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c3tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3TBR)

For information about available fields see [`mod@c3tbr`] module*/
pub type C3TBR = crate::Reg<c3tbr::C3TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c3tbr;
/**C3MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c3mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3MAR)

For information about available fields see [`mod@c3mar`] module*/
pub type C3MAR = crate::Reg<c3mar::C3MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c3mar;
/**C3MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c3mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c3mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C3MDR)

For information about available fields see [`mod@c3mdr`] module*/
pub type C3MDR = crate::Reg<c3mdr::C3MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c3mdr;
/**C4ISR (r) register accessor: MDMA channel 4 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c4isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4ISR)

For information about available fields see [`mod@c4isr`] module*/
pub type C4ISR = crate::Reg<c4isr::C4ISRrs>;
///MDMA channel 4 interrupt/status register
pub mod c4isr;
/**C4IFCR (w) register accessor: MDMA channel 4 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4IFCR)

For information about available fields see [`mod@c4ifcr`] module*/
pub type C4IFCR = crate::Reg<c4ifcr::C4IFCRrs>;
///MDMA channel 4 interrupt flag clear register
pub mod c4ifcr;
/**C4ESR (r) register accessor: MDMA channel 4 error status register

You can [`read`](crate::Reg::read) this register and get [`c4esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4ESR)

For information about available fields see [`mod@c4esr`] module*/
pub type C4ESR = crate::Reg<c4esr::C4ESRrs>;
///MDMA channel 4 error status register
pub mod c4esr;
/**C4CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4CR)

For information about available fields see [`mod@c4cr`] module*/
pub type C4CR = crate::Reg<c4cr::C4CRrs>;
///This register is used to control the concerned channel.
pub mod c4cr;
/**C4TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c4tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4TCR)

For information about available fields see [`mod@c4tcr`] module*/
pub type C4TCR = crate::Reg<c4tcr::C4TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c4tcr;
/**C4BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c4bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4BNDTR)

For information about available fields see [`mod@c4bndtr`] module*/
pub type C4BNDTR = crate::Reg<c4bndtr::C4BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c4bndtr;
/**C4SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c4sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4SAR)

For information about available fields see [`mod@c4sar`] module*/
pub type C4SAR = crate::Reg<c4sar::C4SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c4sar;
/**C4DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c4dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4DAR)

For information about available fields see [`mod@c4dar`] module*/
pub type C4DAR = crate::Reg<c4dar::C4DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c4dar;
/**C4BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c4brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4BRUR)

For information about available fields see [`mod@c4brur`] module*/
pub type C4BRUR = crate::Reg<c4brur::C4BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c4brur;
/**C4LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c4lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4LAR)

For information about available fields see [`mod@c4lar`] module*/
pub type C4LAR = crate::Reg<c4lar::C4LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c4lar;
/**C4TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c4tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4TBR)

For information about available fields see [`mod@c4tbr`] module*/
pub type C4TBR = crate::Reg<c4tbr::C4TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c4tbr;
/**C4MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c4mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4MAR)

For information about available fields see [`mod@c4mar`] module*/
pub type C4MAR = crate::Reg<c4mar::C4MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c4mar;
/**C4MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c4mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c4mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C4MDR)

For information about available fields see [`mod@c4mdr`] module*/
pub type C4MDR = crate::Reg<c4mdr::C4MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c4mdr;
/**C5ISR (r) register accessor: MDMA channel 5 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5ISR)

For information about available fields see [`mod@c5isr`] module*/
pub type C5ISR = crate::Reg<c5isr::C5ISRrs>;
///MDMA channel 5 interrupt/status register
pub mod c5isr;
/**C5IFCR (w) register accessor: MDMA channel 5 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5IFCR)

For information about available fields see [`mod@c5ifcr`] module*/
pub type C5IFCR = crate::Reg<c5ifcr::C5IFCRrs>;
///MDMA channel 5 interrupt flag clear register
pub mod c5ifcr;
/**C5ESR (r) register accessor: MDMA channel 5 error status register

You can [`read`](crate::Reg::read) this register and get [`c5esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5ESR)

For information about available fields see [`mod@c5esr`] module*/
pub type C5ESR = crate::Reg<c5esr::C5ESRrs>;
///MDMA channel 5 error status register
pub mod c5esr;
/**C5CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5CR)

For information about available fields see [`mod@c5cr`] module*/
pub type C5CR = crate::Reg<c5cr::C5CRrs>;
///This register is used to control the concerned channel.
pub mod c5cr;
/**C5TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c5tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5TCR)

For information about available fields see [`mod@c5tcr`] module*/
pub type C5TCR = crate::Reg<c5tcr::C5TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c5tcr;
/**C5BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c5bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5BNDTR)

For information about available fields see [`mod@c5bndtr`] module*/
pub type C5BNDTR = crate::Reg<c5bndtr::C5BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c5bndtr;
/**C5SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c5sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5SAR)

For information about available fields see [`mod@c5sar`] module*/
pub type C5SAR = crate::Reg<c5sar::C5SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c5sar;
/**C5DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c5dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5DAR)

For information about available fields see [`mod@c5dar`] module*/
pub type C5DAR = crate::Reg<c5dar::C5DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c5dar;
/**C5BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c5brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5BRUR)

For information about available fields see [`mod@c5brur`] module*/
pub type C5BRUR = crate::Reg<c5brur::C5BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c5brur;
/**C5LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c5lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5LAR)

For information about available fields see [`mod@c5lar`] module*/
pub type C5LAR = crate::Reg<c5lar::C5LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c5lar;
/**C5TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c5tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5TBR)

For information about available fields see [`mod@c5tbr`] module*/
pub type C5TBR = crate::Reg<c5tbr::C5TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c5tbr;
/**C5MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c5mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5MAR)

For information about available fields see [`mod@c5mar`] module*/
pub type C5MAR = crate::Reg<c5mar::C5MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c5mar;
/**C5MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c5mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c5mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C5MDR)

For information about available fields see [`mod@c5mdr`] module*/
pub type C5MDR = crate::Reg<c5mdr::C5MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c5mdr;
/**C6ISR (r) register accessor: MDMA channel 6 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c6isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6ISR)

For information about available fields see [`mod@c6isr`] module*/
pub type C6ISR = crate::Reg<c6isr::C6ISRrs>;
///MDMA channel 6 interrupt/status register
pub mod c6isr;
/**C6IFCR (w) register accessor: MDMA channel 6 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6IFCR)

For information about available fields see [`mod@c6ifcr`] module*/
pub type C6IFCR = crate::Reg<c6ifcr::C6IFCRrs>;
///MDMA channel 6 interrupt flag clear register
pub mod c6ifcr;
/**C6ESR (r) register accessor: MDMA channel 6 error status register

You can [`read`](crate::Reg::read) this register and get [`c6esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6ESR)

For information about available fields see [`mod@c6esr`] module*/
pub type C6ESR = crate::Reg<c6esr::C6ESRrs>;
///MDMA channel 6 error status register
pub mod c6esr;
/**C6CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6CR)

For information about available fields see [`mod@c6cr`] module*/
pub type C6CR = crate::Reg<c6cr::C6CRrs>;
///This register is used to control the concerned channel.
pub mod c6cr;
/**C6TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c6tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6TCR)

For information about available fields see [`mod@c6tcr`] module*/
pub type C6TCR = crate::Reg<c6tcr::C6TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c6tcr;
/**C6BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c6bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6BNDTR)

For information about available fields see [`mod@c6bndtr`] module*/
pub type C6BNDTR = crate::Reg<c6bndtr::C6BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c6bndtr;
/**C6SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c6sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6SAR)

For information about available fields see [`mod@c6sar`] module*/
pub type C6SAR = crate::Reg<c6sar::C6SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c6sar;
/**C6DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c6dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6DAR)

For information about available fields see [`mod@c6dar`] module*/
pub type C6DAR = crate::Reg<c6dar::C6DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c6dar;
/**C6BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c6brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6BRUR)

For information about available fields see [`mod@c6brur`] module*/
pub type C6BRUR = crate::Reg<c6brur::C6BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c6brur;
/**C6LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c6lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6LAR)

For information about available fields see [`mod@c6lar`] module*/
pub type C6LAR = crate::Reg<c6lar::C6LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c6lar;
/**C6TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c6tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6TBR)

For information about available fields see [`mod@c6tbr`] module*/
pub type C6TBR = crate::Reg<c6tbr::C6TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c6tbr;
/**C6MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c6mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6MAR)

For information about available fields see [`mod@c6mar`] module*/
pub type C6MAR = crate::Reg<c6mar::C6MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c6mar;
/**C6MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c6mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c6mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C6MDR)

For information about available fields see [`mod@c6mdr`] module*/
pub type C6MDR = crate::Reg<c6mdr::C6MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c6mdr;
/**C7ISR (r) register accessor: MDMA channel 7 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c7isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7ISR)

For information about available fields see [`mod@c7isr`] module*/
pub type C7ISR = crate::Reg<c7isr::C7ISRrs>;
///MDMA channel 7 interrupt/status register
pub mod c7isr;
/**C7IFCR (w) register accessor: MDMA channel 7 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7IFCR)

For information about available fields see [`mod@c7ifcr`] module*/
pub type C7IFCR = crate::Reg<c7ifcr::C7IFCRrs>;
///MDMA channel 7 interrupt flag clear register
pub mod c7ifcr;
/**C7ESR (r) register accessor: MDMA channel 7 error status register

You can [`read`](crate::Reg::read) this register and get [`c7esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7ESR)

For information about available fields see [`mod@c7esr`] module*/
pub type C7ESR = crate::Reg<c7esr::C7ESRrs>;
///MDMA channel 7 error status register
pub mod c7esr;
/**C7CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7CR)

For information about available fields see [`mod@c7cr`] module*/
pub type C7CR = crate::Reg<c7cr::C7CRrs>;
///This register is used to control the concerned channel.
pub mod c7cr;
/**C7TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c7tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7TCR)

For information about available fields see [`mod@c7tcr`] module*/
pub type C7TCR = crate::Reg<c7tcr::C7TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c7tcr;
/**C7BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c7bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7BNDTR)

For information about available fields see [`mod@c7bndtr`] module*/
pub type C7BNDTR = crate::Reg<c7bndtr::C7BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c7bndtr;
/**C7SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c7sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7SAR)

For information about available fields see [`mod@c7sar`] module*/
pub type C7SAR = crate::Reg<c7sar::C7SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c7sar;
/**C7DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c7dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7DAR)

For information about available fields see [`mod@c7dar`] module*/
pub type C7DAR = crate::Reg<c7dar::C7DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c7dar;
/**C7BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c7brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7BRUR)

For information about available fields see [`mod@c7brur`] module*/
pub type C7BRUR = crate::Reg<c7brur::C7BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c7brur;
/**C7LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c7lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7LAR)

For information about available fields see [`mod@c7lar`] module*/
pub type C7LAR = crate::Reg<c7lar::C7LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c7lar;
/**C7TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c7tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7TBR)

For information about available fields see [`mod@c7tbr`] module*/
pub type C7TBR = crate::Reg<c7tbr::C7TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c7tbr;
/**C7MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c7mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7MAR)

For information about available fields see [`mod@c7mar`] module*/
pub type C7MAR = crate::Reg<c7mar::C7MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c7mar;
/**C7MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c7mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c7mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C7MDR)

For information about available fields see [`mod@c7mdr`] module*/
pub type C7MDR = crate::Reg<c7mdr::C7MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c7mdr;
/**C8ISR (r) register accessor: MDMA channel 8 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c8isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8ISR)

For information about available fields see [`mod@c8isr`] module*/
pub type C8ISR = crate::Reg<c8isr::C8ISRrs>;
///MDMA channel 8 interrupt/status register
pub mod c8isr;
/**C8IFCR (w) register accessor: MDMA channel 8 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8IFCR)

For information about available fields see [`mod@c8ifcr`] module*/
pub type C8IFCR = crate::Reg<c8ifcr::C8IFCRrs>;
///MDMA channel 8 interrupt flag clear register
pub mod c8ifcr;
/**C8ESR (r) register accessor: MDMA channel 8 error status register

You can [`read`](crate::Reg::read) this register and get [`c8esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8ESR)

For information about available fields see [`mod@c8esr`] module*/
pub type C8ESR = crate::Reg<c8esr::C8ESRrs>;
///MDMA channel 8 error status register
pub mod c8esr;
/**C8CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c8cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8CR)

For information about available fields see [`mod@c8cr`] module*/
pub type C8CR = crate::Reg<c8cr::C8CRrs>;
///This register is used to control the concerned channel.
pub mod c8cr;
/**C8TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c8tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8TCR)

For information about available fields see [`mod@c8tcr`] module*/
pub type C8TCR = crate::Reg<c8tcr::C8TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c8tcr;
/**C8BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c8bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8BNDTR)

For information about available fields see [`mod@c8bndtr`] module*/
pub type C8BNDTR = crate::Reg<c8bndtr::C8BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c8bndtr;
/**C8SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c8sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8SAR)

For information about available fields see [`mod@c8sar`] module*/
pub type C8SAR = crate::Reg<c8sar::C8SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c8sar;
/**C8DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c8dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8DAR)

For information about available fields see [`mod@c8dar`] module*/
pub type C8DAR = crate::Reg<c8dar::C8DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c8dar;
/**C8BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c8brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8BRUR)

For information about available fields see [`mod@c8brur`] module*/
pub type C8BRUR = crate::Reg<c8brur::C8BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c8brur;
/**C8LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c8lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8LAR)

For information about available fields see [`mod@c8lar`] module*/
pub type C8LAR = crate::Reg<c8lar::C8LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c8lar;
/**C8TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c8tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8TBR)

For information about available fields see [`mod@c8tbr`] module*/
pub type C8TBR = crate::Reg<c8tbr::C8TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c8tbr;
/**C8MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c8mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8MAR)

For information about available fields see [`mod@c8mar`] module*/
pub type C8MAR = crate::Reg<c8mar::C8MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c8mar;
/**C8MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c8mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c8mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C8MDR)

For information about available fields see [`mod@c8mdr`] module*/
pub type C8MDR = crate::Reg<c8mdr::C8MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c8mdr;
/**C9ISR (r) register accessor: MDMA channel 9 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c9isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9ISR)

For information about available fields see [`mod@c9isr`] module*/
pub type C9ISR = crate::Reg<c9isr::C9ISRrs>;
///MDMA channel 9 interrupt/status register
pub mod c9isr;
/**C9IFCR (w) register accessor: MDMA channel 9 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9IFCR)

For information about available fields see [`mod@c9ifcr`] module*/
pub type C9IFCR = crate::Reg<c9ifcr::C9IFCRrs>;
///MDMA channel 9 interrupt flag clear register
pub mod c9ifcr;
/**C9ESR (r) register accessor: MDMA channel 9 error status register

You can [`read`](crate::Reg::read) this register and get [`c9esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9ESR)

For information about available fields see [`mod@c9esr`] module*/
pub type C9ESR = crate::Reg<c9esr::C9ESRrs>;
///MDMA channel 9 error status register
pub mod c9esr;
/**C9CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c9cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9CR)

For information about available fields see [`mod@c9cr`] module*/
pub type C9CR = crate::Reg<c9cr::C9CRrs>;
///This register is used to control the concerned channel.
pub mod c9cr;
/**C9TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c9tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9TCR)

For information about available fields see [`mod@c9tcr`] module*/
pub type C9TCR = crate::Reg<c9tcr::C9TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c9tcr;
/**C9BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c9bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9BNDTR)

For information about available fields see [`mod@c9bndtr`] module*/
pub type C9BNDTR = crate::Reg<c9bndtr::C9BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c9bndtr;
/**C9SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c9sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9SAR)

For information about available fields see [`mod@c9sar`] module*/
pub type C9SAR = crate::Reg<c9sar::C9SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c9sar;
/**C9DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c9dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9DAR)

For information about available fields see [`mod@c9dar`] module*/
pub type C9DAR = crate::Reg<c9dar::C9DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c9dar;
/**C9BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c9brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9BRUR)

For information about available fields see [`mod@c9brur`] module*/
pub type C9BRUR = crate::Reg<c9brur::C9BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c9brur;
/**C9LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c9lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9LAR)

For information about available fields see [`mod@c9lar`] module*/
pub type C9LAR = crate::Reg<c9lar::C9LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c9lar;
/**C9TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c9tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9TBR)

For information about available fields see [`mod@c9tbr`] module*/
pub type C9TBR = crate::Reg<c9tbr::C9TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c9tbr;
/**C9MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c9mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9MAR)

For information about available fields see [`mod@c9mar`] module*/
pub type C9MAR = crate::Reg<c9mar::C9MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c9mar;
/**C9MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c9mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c9mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C9MDR)

For information about available fields see [`mod@c9mdr`] module*/
pub type C9MDR = crate::Reg<c9mdr::C9MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c9mdr;
/**C10ISR (r) register accessor: MDMA channel 10 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c10isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10ISR)

For information about available fields see [`mod@c10isr`] module*/
pub type C10ISR = crate::Reg<c10isr::C10ISRrs>;
///MDMA channel 10 interrupt/status register
pub mod c10isr;
/**C10IFCR (w) register accessor: MDMA channel 10 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10IFCR)

For information about available fields see [`mod@c10ifcr`] module*/
pub type C10IFCR = crate::Reg<c10ifcr::C10IFCRrs>;
///MDMA channel 10 interrupt flag clear register
pub mod c10ifcr;
/**C10ESR (r) register accessor: MDMA channel 10 error status register

You can [`read`](crate::Reg::read) this register and get [`c10esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10ESR)

For information about available fields see [`mod@c10esr`] module*/
pub type C10ESR = crate::Reg<c10esr::C10ESRrs>;
///MDMA channel 10 error status register
pub mod c10esr;
/**C10CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c10cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10CR)

For information about available fields see [`mod@c10cr`] module*/
pub type C10CR = crate::Reg<c10cr::C10CRrs>;
///This register is used to control the concerned channel.
pub mod c10cr;
/**C10TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c10tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10TCR)

For information about available fields see [`mod@c10tcr`] module*/
pub type C10TCR = crate::Reg<c10tcr::C10TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c10tcr;
/**C10BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c10bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10BNDTR)

For information about available fields see [`mod@c10bndtr`] module*/
pub type C10BNDTR = crate::Reg<c10bndtr::C10BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c10bndtr;
/**C10SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c10sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10SAR)

For information about available fields see [`mod@c10sar`] module*/
pub type C10SAR = crate::Reg<c10sar::C10SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c10sar;
/**C10DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c10dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10DAR)

For information about available fields see [`mod@c10dar`] module*/
pub type C10DAR = crate::Reg<c10dar::C10DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c10dar;
/**C10BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c10brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10BRUR)

For information about available fields see [`mod@c10brur`] module*/
pub type C10BRUR = crate::Reg<c10brur::C10BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c10brur;
/**C10LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c10lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10LAR)

For information about available fields see [`mod@c10lar`] module*/
pub type C10LAR = crate::Reg<c10lar::C10LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c10lar;
/**C10TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c10tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10TBR)

For information about available fields see [`mod@c10tbr`] module*/
pub type C10TBR = crate::Reg<c10tbr::C10TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c10tbr;
/**C10MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c10mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10MAR)

For information about available fields see [`mod@c10mar`] module*/
pub type C10MAR = crate::Reg<c10mar::C10MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c10mar;
/**C10MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c10mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c10mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C10MDR)

For information about available fields see [`mod@c10mdr`] module*/
pub type C10MDR = crate::Reg<c10mdr::C10MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c10mdr;
/**C11ISR (r) register accessor: MDMA channel 11 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c11isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11ISR)

For information about available fields see [`mod@c11isr`] module*/
pub type C11ISR = crate::Reg<c11isr::C11ISRrs>;
///MDMA channel 11 interrupt/status register
pub mod c11isr;
/**C11IFCR (w) register accessor: MDMA channel 11 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11IFCR)

For information about available fields see [`mod@c11ifcr`] module*/
pub type C11IFCR = crate::Reg<c11ifcr::C11IFCRrs>;
///MDMA channel 11 interrupt flag clear register
pub mod c11ifcr;
/**C11ESR (r) register accessor: MDMA channel 11 error status register

You can [`read`](crate::Reg::read) this register and get [`c11esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11ESR)

For information about available fields see [`mod@c11esr`] module*/
pub type C11ESR = crate::Reg<c11esr::C11ESRrs>;
///MDMA channel 11 error status register
pub mod c11esr;
/**C11CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c11cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11CR)

For information about available fields see [`mod@c11cr`] module*/
pub type C11CR = crate::Reg<c11cr::C11CRrs>;
///This register is used to control the concerned channel.
pub mod c11cr;
/**C11TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c11tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11TCR)

For information about available fields see [`mod@c11tcr`] module*/
pub type C11TCR = crate::Reg<c11tcr::C11TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c11tcr;
/**C11BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c11bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11BNDTR)

For information about available fields see [`mod@c11bndtr`] module*/
pub type C11BNDTR = crate::Reg<c11bndtr::C11BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c11bndtr;
/**C11SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c11sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11SAR)

For information about available fields see [`mod@c11sar`] module*/
pub type C11SAR = crate::Reg<c11sar::C11SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c11sar;
/**C11DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c11dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11DAR)

For information about available fields see [`mod@c11dar`] module*/
pub type C11DAR = crate::Reg<c11dar::C11DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c11dar;
/**C11BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c11brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11BRUR)

For information about available fields see [`mod@c11brur`] module*/
pub type C11BRUR = crate::Reg<c11brur::C11BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c11brur;
/**C11LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c11lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11LAR)

For information about available fields see [`mod@c11lar`] module*/
pub type C11LAR = crate::Reg<c11lar::C11LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c11lar;
/**C11TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c11tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11TBR)

For information about available fields see [`mod@c11tbr`] module*/
pub type C11TBR = crate::Reg<c11tbr::C11TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c11tbr;
/**C11MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c11mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11MAR)

For information about available fields see [`mod@c11mar`] module*/
pub type C11MAR = crate::Reg<c11mar::C11MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c11mar;
/**C11MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c11mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c11mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C11MDR)

For information about available fields see [`mod@c11mdr`] module*/
pub type C11MDR = crate::Reg<c11mdr::C11MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c11mdr;
/**C12ISR (r) register accessor: MDMA channel 12 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c12isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12ISR)

For information about available fields see [`mod@c12isr`] module*/
pub type C12ISR = crate::Reg<c12isr::C12ISRrs>;
///MDMA channel 12 interrupt/status register
pub mod c12isr;
/**C12IFCR (w) register accessor: MDMA channel 12 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12IFCR)

For information about available fields see [`mod@c12ifcr`] module*/
pub type C12IFCR = crate::Reg<c12ifcr::C12IFCRrs>;
///MDMA channel 12 interrupt flag clear register
pub mod c12ifcr;
/**C12ESR (r) register accessor: MDMA channel 12 error status register

You can [`read`](crate::Reg::read) this register and get [`c12esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12ESR)

For information about available fields see [`mod@c12esr`] module*/
pub type C12ESR = crate::Reg<c12esr::C12ESRrs>;
///MDMA channel 12 error status register
pub mod c12esr;
/**C12CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c12cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12CR)

For information about available fields see [`mod@c12cr`] module*/
pub type C12CR = crate::Reg<c12cr::C12CRrs>;
///This register is used to control the concerned channel.
pub mod c12cr;
/**C12TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c12tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12TCR)

For information about available fields see [`mod@c12tcr`] module*/
pub type C12TCR = crate::Reg<c12tcr::C12TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c12tcr;
/**C12BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c12bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12BNDTR)

For information about available fields see [`mod@c12bndtr`] module*/
pub type C12BNDTR = crate::Reg<c12bndtr::C12BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c12bndtr;
/**C12SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c12sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12SAR)

For information about available fields see [`mod@c12sar`] module*/
pub type C12SAR = crate::Reg<c12sar::C12SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c12sar;
/**C12DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c12dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12DAR)

For information about available fields see [`mod@c12dar`] module*/
pub type C12DAR = crate::Reg<c12dar::C12DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c12dar;
/**C12BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c12brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12BRUR)

For information about available fields see [`mod@c12brur`] module*/
pub type C12BRUR = crate::Reg<c12brur::C12BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c12brur;
/**C12LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c12lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12LAR)

For information about available fields see [`mod@c12lar`] module*/
pub type C12LAR = crate::Reg<c12lar::C12LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c12lar;
/**C12TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c12tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12TBR)

For information about available fields see [`mod@c12tbr`] module*/
pub type C12TBR = crate::Reg<c12tbr::C12TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c12tbr;
/**C12MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c12mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12MAR)

For information about available fields see [`mod@c12mar`] module*/
pub type C12MAR = crate::Reg<c12mar::C12MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c12mar;
/**C12MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c12mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c12mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C12MDR)

For information about available fields see [`mod@c12mdr`] module*/
pub type C12MDR = crate::Reg<c12mdr::C12MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c12mdr;
/**C13ISR (r) register accessor: MDMA channel 13 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c13isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13ISR)

For information about available fields see [`mod@c13isr`] module*/
pub type C13ISR = crate::Reg<c13isr::C13ISRrs>;
///MDMA channel 13 interrupt/status register
pub mod c13isr;
/**C13IFCR (w) register accessor: MDMA channel 13 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13IFCR)

For information about available fields see [`mod@c13ifcr`] module*/
pub type C13IFCR = crate::Reg<c13ifcr::C13IFCRrs>;
///MDMA channel 13 interrupt flag clear register
pub mod c13ifcr;
/**C13ESR (r) register accessor: MDMA channel 13 error status register

You can [`read`](crate::Reg::read) this register and get [`c13esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13ESR)

For information about available fields see [`mod@c13esr`] module*/
pub type C13ESR = crate::Reg<c13esr::C13ESRrs>;
///MDMA channel 13 error status register
pub mod c13esr;
/**C13CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c13cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13CR)

For information about available fields see [`mod@c13cr`] module*/
pub type C13CR = crate::Reg<c13cr::C13CRrs>;
///This register is used to control the concerned channel.
pub mod c13cr;
/**C13TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c13tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13TCR)

For information about available fields see [`mod@c13tcr`] module*/
pub type C13TCR = crate::Reg<c13tcr::C13TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c13tcr;
/**C13BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c13bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13BNDTR)

For information about available fields see [`mod@c13bndtr`] module*/
pub type C13BNDTR = crate::Reg<c13bndtr::C13BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c13bndtr;
/**C13SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c13sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13SAR)

For information about available fields see [`mod@c13sar`] module*/
pub type C13SAR = crate::Reg<c13sar::C13SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c13sar;
/**C13DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c13dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13DAR)

For information about available fields see [`mod@c13dar`] module*/
pub type C13DAR = crate::Reg<c13dar::C13DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c13dar;
/**C13BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c13brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13BRUR)

For information about available fields see [`mod@c13brur`] module*/
pub type C13BRUR = crate::Reg<c13brur::C13BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c13brur;
/**C13LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c13lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13LAR)

For information about available fields see [`mod@c13lar`] module*/
pub type C13LAR = crate::Reg<c13lar::C13LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c13lar;
/**C13TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c13tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13TBR)

For information about available fields see [`mod@c13tbr`] module*/
pub type C13TBR = crate::Reg<c13tbr::C13TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c13tbr;
/**C13MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c13mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13MAR)

For information about available fields see [`mod@c13mar`] module*/
pub type C13MAR = crate::Reg<c13mar::C13MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c13mar;
/**C13MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c13mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c13mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C13MDR)

For information about available fields see [`mod@c13mdr`] module*/
pub type C13MDR = crate::Reg<c13mdr::C13MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c13mdr;
/**C14ISR (r) register accessor: MDMA channel 14 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c14isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14ISR)

For information about available fields see [`mod@c14isr`] module*/
pub type C14ISR = crate::Reg<c14isr::C14ISRrs>;
///MDMA channel 14 interrupt/status register
pub mod c14isr;
/**C14IFCR (w) register accessor: MDMA channel 14 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14IFCR)

For information about available fields see [`mod@c14ifcr`] module*/
pub type C14IFCR = crate::Reg<c14ifcr::C14IFCRrs>;
///MDMA channel 14 interrupt flag clear register
pub mod c14ifcr;
/**C14ESR (r) register accessor: MDMA channel 14 error status register

You can [`read`](crate::Reg::read) this register and get [`c14esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14ESR)

For information about available fields see [`mod@c14esr`] module*/
pub type C14ESR = crate::Reg<c14esr::C14ESRrs>;
///MDMA channel 14 error status register
pub mod c14esr;
/**C14CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c14cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14CR)

For information about available fields see [`mod@c14cr`] module*/
pub type C14CR = crate::Reg<c14cr::C14CRrs>;
///This register is used to control the concerned channel.
pub mod c14cr;
/**C14TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c14tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14TCR)

For information about available fields see [`mod@c14tcr`] module*/
pub type C14TCR = crate::Reg<c14tcr::C14TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c14tcr;
/**C14BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c14bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14BNDTR)

For information about available fields see [`mod@c14bndtr`] module*/
pub type C14BNDTR = crate::Reg<c14bndtr::C14BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c14bndtr;
/**C14SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c14sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14SAR)

For information about available fields see [`mod@c14sar`] module*/
pub type C14SAR = crate::Reg<c14sar::C14SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c14sar;
/**C14DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c14dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14DAR)

For information about available fields see [`mod@c14dar`] module*/
pub type C14DAR = crate::Reg<c14dar::C14DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c14dar;
/**C14BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c14brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14BRUR)

For information about available fields see [`mod@c14brur`] module*/
pub type C14BRUR = crate::Reg<c14brur::C14BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c14brur;
/**C14LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c14lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14LAR)

For information about available fields see [`mod@c14lar`] module*/
pub type C14LAR = crate::Reg<c14lar::C14LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c14lar;
/**C14TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c14tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14TBR)

For information about available fields see [`mod@c14tbr`] module*/
pub type C14TBR = crate::Reg<c14tbr::C14TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c14tbr;
/**C14MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c14mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14MAR)

For information about available fields see [`mod@c14mar`] module*/
pub type C14MAR = crate::Reg<c14mar::C14MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c14mar;
/**C14MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c14mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c14mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C14MDR)

For information about available fields see [`mod@c14mdr`] module*/
pub type C14MDR = crate::Reg<c14mdr::C14MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c14mdr;
/**C15ISR (r) register accessor: MDMA channel 15 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c15isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15ISR)

For information about available fields see [`mod@c15isr`] module*/
pub type C15ISR = crate::Reg<c15isr::C15ISRrs>;
///MDMA channel 15 interrupt/status register
pub mod c15isr;
/**C15IFCR (w) register accessor: MDMA channel 15 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15IFCR)

For information about available fields see [`mod@c15ifcr`] module*/
pub type C15IFCR = crate::Reg<c15ifcr::C15IFCRrs>;
///MDMA channel 15 interrupt flag clear register
pub mod c15ifcr;
/**C15ESR (r) register accessor: MDMA channel 15 error status register

You can [`read`](crate::Reg::read) this register and get [`c15esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15ESR)

For information about available fields see [`mod@c15esr`] module*/
pub type C15ESR = crate::Reg<c15esr::C15ESRrs>;
///MDMA channel 15 error status register
pub mod c15esr;
/**C15CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c15cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15CR)

For information about available fields see [`mod@c15cr`] module*/
pub type C15CR = crate::Reg<c15cr::C15CRrs>;
///This register is used to control the concerned channel.
pub mod c15cr;
/**C15TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c15tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15TCR)

For information about available fields see [`mod@c15tcr`] module*/
pub type C15TCR = crate::Reg<c15tcr::C15TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c15tcr;
/**C15BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c15bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15BNDTR)

For information about available fields see [`mod@c15bndtr`] module*/
pub type C15BNDTR = crate::Reg<c15bndtr::C15BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c15bndtr;
/**C15SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c15sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15SAR)

For information about available fields see [`mod@c15sar`] module*/
pub type C15SAR = crate::Reg<c15sar::C15SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c15sar;
/**C15DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c15dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15DAR)

For information about available fields see [`mod@c15dar`] module*/
pub type C15DAR = crate::Reg<c15dar::C15DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c15dar;
/**C15BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c15brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15BRUR)

For information about available fields see [`mod@c15brur`] module*/
pub type C15BRUR = crate::Reg<c15brur::C15BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c15brur;
/**C15LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c15lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15LAR)

For information about available fields see [`mod@c15lar`] module*/
pub type C15LAR = crate::Reg<c15lar::C15LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c15lar;
/**C15TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c15tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15TBR)

For information about available fields see [`mod@c15tbr`] module*/
pub type C15TBR = crate::Reg<c15tbr::C15TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c15tbr;
/**C15MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c15mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15MAR)

For information about available fields see [`mod@c15mar`] module*/
pub type C15MAR = crate::Reg<c15mar::C15MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c15mar;
/**C15MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c15mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c15mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C15MDR)

For information about available fields see [`mod@c15mdr`] module*/
pub type C15MDR = crate::Reg<c15mdr::C15MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c15mdr;
/**C16ISR (r) register accessor: MDMA channel 16 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c16isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16ISR)

For information about available fields see [`mod@c16isr`] module*/
pub type C16ISR = crate::Reg<c16isr::C16ISRrs>;
///MDMA channel 16 interrupt/status register
pub mod c16isr;
/**C16IFCR (w) register accessor: MDMA channel 16 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16IFCR)

For information about available fields see [`mod@c16ifcr`] module*/
pub type C16IFCR = crate::Reg<c16ifcr::C16IFCRrs>;
///MDMA channel 16 interrupt flag clear register
pub mod c16ifcr;
/**C16ESR (r) register accessor: MDMA channel 16 error status register

You can [`read`](crate::Reg::read) this register and get [`c16esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16ESR)

For information about available fields see [`mod@c16esr`] module*/
pub type C16ESR = crate::Reg<c16esr::C16ESRrs>;
///MDMA channel 16 error status register
pub mod c16esr;
/**C16CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c16cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16CR)

For information about available fields see [`mod@c16cr`] module*/
pub type C16CR = crate::Reg<c16cr::C16CRrs>;
///This register is used to control the concerned channel.
pub mod c16cr;
/**C16TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c16tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16TCR)

For information about available fields see [`mod@c16tcr`] module*/
pub type C16TCR = crate::Reg<c16tcr::C16TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c16tcr;
/**C16BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c16bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16BNDTR)

For information about available fields see [`mod@c16bndtr`] module*/
pub type C16BNDTR = crate::Reg<c16bndtr::C16BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c16bndtr;
/**C16SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c16sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16SAR)

For information about available fields see [`mod@c16sar`] module*/
pub type C16SAR = crate::Reg<c16sar::C16SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c16sar;
/**C16DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c16dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16DAR)

For information about available fields see [`mod@c16dar`] module*/
pub type C16DAR = crate::Reg<c16dar::C16DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c16dar;
/**C16BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c16brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16BRUR)

For information about available fields see [`mod@c16brur`] module*/
pub type C16BRUR = crate::Reg<c16brur::C16BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c16brur;
/**C16LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c16lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16LAR)

For information about available fields see [`mod@c16lar`] module*/
pub type C16LAR = crate::Reg<c16lar::C16LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c16lar;
/**C16TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c16tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16TBR)

For information about available fields see [`mod@c16tbr`] module*/
pub type C16TBR = crate::Reg<c16tbr::C16TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c16tbr;
/**C16MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c16mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16MAR)

For information about available fields see [`mod@c16mar`] module*/
pub type C16MAR = crate::Reg<c16mar::C16MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c16mar;
/**C16MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c16mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c16mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C16MDR)

For information about available fields see [`mod@c16mdr`] module*/
pub type C16MDR = crate::Reg<c16mdr::C16MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c16mdr;
/**C17ISR (r) register accessor: MDMA channel 17 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c17isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17ISR)

For information about available fields see [`mod@c17isr`] module*/
pub type C17ISR = crate::Reg<c17isr::C17ISRrs>;
///MDMA channel 17 interrupt/status register
pub mod c17isr;
/**C17IFCR (w) register accessor: MDMA channel 17 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17IFCR)

For information about available fields see [`mod@c17ifcr`] module*/
pub type C17IFCR = crate::Reg<c17ifcr::C17IFCRrs>;
///MDMA channel 17 interrupt flag clear register
pub mod c17ifcr;
/**C17ESR (r) register accessor: MDMA channel 17 error status register

You can [`read`](crate::Reg::read) this register and get [`c17esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17ESR)

For information about available fields see [`mod@c17esr`] module*/
pub type C17ESR = crate::Reg<c17esr::C17ESRrs>;
///MDMA channel 17 error status register
pub mod c17esr;
/**C17CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c17cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17CR)

For information about available fields see [`mod@c17cr`] module*/
pub type C17CR = crate::Reg<c17cr::C17CRrs>;
///This register is used to control the concerned channel.
pub mod c17cr;
/**C17TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c17tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17TCR)

For information about available fields see [`mod@c17tcr`] module*/
pub type C17TCR = crate::Reg<c17tcr::C17TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c17tcr;
/**C17BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c17bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17BNDTR)

For information about available fields see [`mod@c17bndtr`] module*/
pub type C17BNDTR = crate::Reg<c17bndtr::C17BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c17bndtr;
/**C17SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c17sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17SAR)

For information about available fields see [`mod@c17sar`] module*/
pub type C17SAR = crate::Reg<c17sar::C17SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c17sar;
/**C17DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c17dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17DAR)

For information about available fields see [`mod@c17dar`] module*/
pub type C17DAR = crate::Reg<c17dar::C17DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c17dar;
/**C17BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c17brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17BRUR)

For information about available fields see [`mod@c17brur`] module*/
pub type C17BRUR = crate::Reg<c17brur::C17BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c17brur;
/**C17LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c17lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17LAR)

For information about available fields see [`mod@c17lar`] module*/
pub type C17LAR = crate::Reg<c17lar::C17LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c17lar;
/**C17TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c17tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17TBR)

For information about available fields see [`mod@c17tbr`] module*/
pub type C17TBR = crate::Reg<c17tbr::C17TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c17tbr;
/**C17MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c17mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17MAR)

For information about available fields see [`mod@c17mar`] module*/
pub type C17MAR = crate::Reg<c17mar::C17MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c17mar;
/**C17MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c17mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c17mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C17MDR)

For information about available fields see [`mod@c17mdr`] module*/
pub type C17MDR = crate::Reg<c17mdr::C17MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c17mdr;
/**C18ISR (r) register accessor: MDMA channel 18 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c18isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18ISR)

For information about available fields see [`mod@c18isr`] module*/
pub type C18ISR = crate::Reg<c18isr::C18ISRrs>;
///MDMA channel 18 interrupt/status register
pub mod c18isr;
/**C18IFCR (w) register accessor: MDMA channel 18 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18IFCR)

For information about available fields see [`mod@c18ifcr`] module*/
pub type C18IFCR = crate::Reg<c18ifcr::C18IFCRrs>;
///MDMA channel 18 interrupt flag clear register
pub mod c18ifcr;
/**C18ESR (r) register accessor: MDMA channel 18 error status register

You can [`read`](crate::Reg::read) this register and get [`c18esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18ESR)

For information about available fields see [`mod@c18esr`] module*/
pub type C18ESR = crate::Reg<c18esr::C18ESRrs>;
///MDMA channel 18 error status register
pub mod c18esr;
/**C18CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c18cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18CR)

For information about available fields see [`mod@c18cr`] module*/
pub type C18CR = crate::Reg<c18cr::C18CRrs>;
///This register is used to control the concerned channel.
pub mod c18cr;
/**C18TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c18tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18TCR)

For information about available fields see [`mod@c18tcr`] module*/
pub type C18TCR = crate::Reg<c18tcr::C18TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c18tcr;
/**C18BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c18bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18BNDTR)

For information about available fields see [`mod@c18bndtr`] module*/
pub type C18BNDTR = crate::Reg<c18bndtr::C18BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c18bndtr;
/**C18SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c18sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18SAR)

For information about available fields see [`mod@c18sar`] module*/
pub type C18SAR = crate::Reg<c18sar::C18SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c18sar;
/**C18DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c18dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18DAR)

For information about available fields see [`mod@c18dar`] module*/
pub type C18DAR = crate::Reg<c18dar::C18DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c18dar;
/**C18BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c18brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18BRUR)

For information about available fields see [`mod@c18brur`] module*/
pub type C18BRUR = crate::Reg<c18brur::C18BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c18brur;
/**C18LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c18lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18LAR)

For information about available fields see [`mod@c18lar`] module*/
pub type C18LAR = crate::Reg<c18lar::C18LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c18lar;
/**C18TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c18tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18TBR)

For information about available fields see [`mod@c18tbr`] module*/
pub type C18TBR = crate::Reg<c18tbr::C18TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c18tbr;
/**C18MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c18mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18MAR)

For information about available fields see [`mod@c18mar`] module*/
pub type C18MAR = crate::Reg<c18mar::C18MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c18mar;
/**C18MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c18mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c18mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C18MDR)

For information about available fields see [`mod@c18mdr`] module*/
pub type C18MDR = crate::Reg<c18mdr::C18MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c18mdr;
/**C19ISR (r) register accessor: MDMA channel 19 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c19isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19ISR)

For information about available fields see [`mod@c19isr`] module*/
pub type C19ISR = crate::Reg<c19isr::C19ISRrs>;
///MDMA channel 19 interrupt/status register
pub mod c19isr;
/**C19IFCR (w) register accessor: MDMA channel 19 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19IFCR)

For information about available fields see [`mod@c19ifcr`] module*/
pub type C19IFCR = crate::Reg<c19ifcr::C19IFCRrs>;
///MDMA channel 19 interrupt flag clear register
pub mod c19ifcr;
/**C19ESR (r) register accessor: MDMA channel 19 error status register

You can [`read`](crate::Reg::read) this register and get [`c19esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19ESR)

For information about available fields see [`mod@c19esr`] module*/
pub type C19ESR = crate::Reg<c19esr::C19ESRrs>;
///MDMA channel 19 error status register
pub mod c19esr;
/**C19CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c19cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19CR)

For information about available fields see [`mod@c19cr`] module*/
pub type C19CR = crate::Reg<c19cr::C19CRrs>;
///This register is used to control the concerned channel.
pub mod c19cr;
/**C19TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c19tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19TCR)

For information about available fields see [`mod@c19tcr`] module*/
pub type C19TCR = crate::Reg<c19tcr::C19TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c19tcr;
/**C19BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c19bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19BNDTR)

For information about available fields see [`mod@c19bndtr`] module*/
pub type C19BNDTR = crate::Reg<c19bndtr::C19BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c19bndtr;
/**C19SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c19sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19SAR)

For information about available fields see [`mod@c19sar`] module*/
pub type C19SAR = crate::Reg<c19sar::C19SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c19sar;
/**C19DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c19dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19DAR)

For information about available fields see [`mod@c19dar`] module*/
pub type C19DAR = crate::Reg<c19dar::C19DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c19dar;
/**C19BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c19brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19BRUR)

For information about available fields see [`mod@c19brur`] module*/
pub type C19BRUR = crate::Reg<c19brur::C19BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c19brur;
/**C19LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c19lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19LAR)

For information about available fields see [`mod@c19lar`] module*/
pub type C19LAR = crate::Reg<c19lar::C19LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c19lar;
/**C19TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c19tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19TBR)

For information about available fields see [`mod@c19tbr`] module*/
pub type C19TBR = crate::Reg<c19tbr::C19TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c19tbr;
/**C19MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c19mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19MAR)

For information about available fields see [`mod@c19mar`] module*/
pub type C19MAR = crate::Reg<c19mar::C19MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c19mar;
/**C19MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c19mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c19mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C19MDR)

For information about available fields see [`mod@c19mdr`] module*/
pub type C19MDR = crate::Reg<c19mdr::C19MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c19mdr;
/**C20ISR (r) register accessor: MDMA channel 20 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c20isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20ISR)

For information about available fields see [`mod@c20isr`] module*/
pub type C20ISR = crate::Reg<c20isr::C20ISRrs>;
///MDMA channel 20 interrupt/status register
pub mod c20isr;
/**C20IFCR (w) register accessor: MDMA channel 20 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20IFCR)

For information about available fields see [`mod@c20ifcr`] module*/
pub type C20IFCR = crate::Reg<c20ifcr::C20IFCRrs>;
///MDMA channel 20 interrupt flag clear register
pub mod c20ifcr;
/**C20ESR (r) register accessor: MDMA channel 20 error status register

You can [`read`](crate::Reg::read) this register and get [`c20esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20ESR)

For information about available fields see [`mod@c20esr`] module*/
pub type C20ESR = crate::Reg<c20esr::C20ESRrs>;
///MDMA channel 20 error status register
pub mod c20esr;
/**C20CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c20cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20CR)

For information about available fields see [`mod@c20cr`] module*/
pub type C20CR = crate::Reg<c20cr::C20CRrs>;
///This register is used to control the concerned channel.
pub mod c20cr;
/**C20TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c20tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20TCR)

For information about available fields see [`mod@c20tcr`] module*/
pub type C20TCR = crate::Reg<c20tcr::C20TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c20tcr;
/**C20BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c20bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20BNDTR)

For information about available fields see [`mod@c20bndtr`] module*/
pub type C20BNDTR = crate::Reg<c20bndtr::C20BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c20bndtr;
/**C20SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c20sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20SAR)

For information about available fields see [`mod@c20sar`] module*/
pub type C20SAR = crate::Reg<c20sar::C20SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c20sar;
/**C20DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c20dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20DAR)

For information about available fields see [`mod@c20dar`] module*/
pub type C20DAR = crate::Reg<c20dar::C20DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c20dar;
/**C20BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c20brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20BRUR)

For information about available fields see [`mod@c20brur`] module*/
pub type C20BRUR = crate::Reg<c20brur::C20BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c20brur;
/**C20LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c20lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20LAR)

For information about available fields see [`mod@c20lar`] module*/
pub type C20LAR = crate::Reg<c20lar::C20LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c20lar;
/**C20TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c20tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20TBR)

For information about available fields see [`mod@c20tbr`] module*/
pub type C20TBR = crate::Reg<c20tbr::C20TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c20tbr;
/**C20MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c20mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20MAR)

For information about available fields see [`mod@c20mar`] module*/
pub type C20MAR = crate::Reg<c20mar::C20MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c20mar;
/**C20MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c20mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c20mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C20MDR)

For information about available fields see [`mod@c20mdr`] module*/
pub type C20MDR = crate::Reg<c20mdr::C20MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c20mdr;
/**C21ISR (r) register accessor: MDMA channel 21 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c21isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21ISR)

For information about available fields see [`mod@c21isr`] module*/
pub type C21ISR = crate::Reg<c21isr::C21ISRrs>;
///MDMA channel 21 interrupt/status register
pub mod c21isr;
/**C21IFCR (w) register accessor: MDMA channel 21 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21IFCR)

For information about available fields see [`mod@c21ifcr`] module*/
pub type C21IFCR = crate::Reg<c21ifcr::C21IFCRrs>;
///MDMA channel 21 interrupt flag clear register
pub mod c21ifcr;
/**C21ESR (r) register accessor: MDMA channel 21 error status register

You can [`read`](crate::Reg::read) this register and get [`c21esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21ESR)

For information about available fields see [`mod@c21esr`] module*/
pub type C21ESR = crate::Reg<c21esr::C21ESRrs>;
///MDMA channel 21 error status register
pub mod c21esr;
/**C21CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c21cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21CR)

For information about available fields see [`mod@c21cr`] module*/
pub type C21CR = crate::Reg<c21cr::C21CRrs>;
///This register is used to control the concerned channel.
pub mod c21cr;
/**C21TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c21tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21TCR)

For information about available fields see [`mod@c21tcr`] module*/
pub type C21TCR = crate::Reg<c21tcr::C21TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c21tcr;
/**C21BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c21bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21BNDTR)

For information about available fields see [`mod@c21bndtr`] module*/
pub type C21BNDTR = crate::Reg<c21bndtr::C21BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c21bndtr;
/**C21SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c21sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21SAR)

For information about available fields see [`mod@c21sar`] module*/
pub type C21SAR = crate::Reg<c21sar::C21SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c21sar;
/**C21DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c21dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21DAR)

For information about available fields see [`mod@c21dar`] module*/
pub type C21DAR = crate::Reg<c21dar::C21DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c21dar;
/**C21BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c21brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21BRUR)

For information about available fields see [`mod@c21brur`] module*/
pub type C21BRUR = crate::Reg<c21brur::C21BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c21brur;
/**C21LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c21lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21LAR)

For information about available fields see [`mod@c21lar`] module*/
pub type C21LAR = crate::Reg<c21lar::C21LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c21lar;
/**C21TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c21tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21TBR)

For information about available fields see [`mod@c21tbr`] module*/
pub type C21TBR = crate::Reg<c21tbr::C21TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c21tbr;
/**C21MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c21mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21MAR)

For information about available fields see [`mod@c21mar`] module*/
pub type C21MAR = crate::Reg<c21mar::C21MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c21mar;
/**C21MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c21mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c21mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C21MDR)

For information about available fields see [`mod@c21mdr`] module*/
pub type C21MDR = crate::Reg<c21mdr::C21MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c21mdr;
/**C22ISR (r) register accessor: MDMA channel 22 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c22isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22ISR)

For information about available fields see [`mod@c22isr`] module*/
pub type C22ISR = crate::Reg<c22isr::C22ISRrs>;
///MDMA channel 22 interrupt/status register
pub mod c22isr;
/**C22IFCR (w) register accessor: MDMA channel 22 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22IFCR)

For information about available fields see [`mod@c22ifcr`] module*/
pub type C22IFCR = crate::Reg<c22ifcr::C22IFCRrs>;
///MDMA channel 22 interrupt flag clear register
pub mod c22ifcr;
/**C22ESR (r) register accessor: MDMA channel 22 error status register

You can [`read`](crate::Reg::read) this register and get [`c22esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22ESR)

For information about available fields see [`mod@c22esr`] module*/
pub type C22ESR = crate::Reg<c22esr::C22ESRrs>;
///MDMA channel 22 error status register
pub mod c22esr;
/**C22CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c22cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22CR)

For information about available fields see [`mod@c22cr`] module*/
pub type C22CR = crate::Reg<c22cr::C22CRrs>;
///This register is used to control the concerned channel.
pub mod c22cr;
/**C22TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c22tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22TCR)

For information about available fields see [`mod@c22tcr`] module*/
pub type C22TCR = crate::Reg<c22tcr::C22TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c22tcr;
/**C22BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c22bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22BNDTR)

For information about available fields see [`mod@c22bndtr`] module*/
pub type C22BNDTR = crate::Reg<c22bndtr::C22BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c22bndtr;
/**C22SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c22sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22SAR)

For information about available fields see [`mod@c22sar`] module*/
pub type C22SAR = crate::Reg<c22sar::C22SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c22sar;
/**C22DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c22dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22DAR)

For information about available fields see [`mod@c22dar`] module*/
pub type C22DAR = crate::Reg<c22dar::C22DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c22dar;
/**C22BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c22brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22BRUR)

For information about available fields see [`mod@c22brur`] module*/
pub type C22BRUR = crate::Reg<c22brur::C22BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c22brur;
/**C22LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c22lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22LAR)

For information about available fields see [`mod@c22lar`] module*/
pub type C22LAR = crate::Reg<c22lar::C22LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c22lar;
/**C22TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c22tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22TBR)

For information about available fields see [`mod@c22tbr`] module*/
pub type C22TBR = crate::Reg<c22tbr::C22TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c22tbr;
/**C22MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c22mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22MAR)

For information about available fields see [`mod@c22mar`] module*/
pub type C22MAR = crate::Reg<c22mar::C22MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c22mar;
/**C22MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c22mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c22mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C22MDR)

For information about available fields see [`mod@c22mdr`] module*/
pub type C22MDR = crate::Reg<c22mdr::C22MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c22mdr;
/**C23ISR (r) register accessor: MDMA channel 23 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c23isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23ISR)

For information about available fields see [`mod@c23isr`] module*/
pub type C23ISR = crate::Reg<c23isr::C23ISRrs>;
///MDMA channel 23 interrupt/status register
pub mod c23isr;
/**C23IFCR (w) register accessor: MDMA channel 23 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23IFCR)

For information about available fields see [`mod@c23ifcr`] module*/
pub type C23IFCR = crate::Reg<c23ifcr::C23IFCRrs>;
///MDMA channel 23 interrupt flag clear register
pub mod c23ifcr;
/**C23ESR (r) register accessor: MDMA channel 23 error status register

You can [`read`](crate::Reg::read) this register and get [`c23esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23ESR)

For information about available fields see [`mod@c23esr`] module*/
pub type C23ESR = crate::Reg<c23esr::C23ESRrs>;
///MDMA channel 23 error status register
pub mod c23esr;
/**C23CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c23cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23CR)

For information about available fields see [`mod@c23cr`] module*/
pub type C23CR = crate::Reg<c23cr::C23CRrs>;
///This register is used to control the concerned channel.
pub mod c23cr;
/**C23TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c23tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23TCR)

For information about available fields see [`mod@c23tcr`] module*/
pub type C23TCR = crate::Reg<c23tcr::C23TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c23tcr;
/**C23BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c23bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23BNDTR)

For information about available fields see [`mod@c23bndtr`] module*/
pub type C23BNDTR = crate::Reg<c23bndtr::C23BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c23bndtr;
/**C23SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c23sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23SAR)

For information about available fields see [`mod@c23sar`] module*/
pub type C23SAR = crate::Reg<c23sar::C23SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c23sar;
/**C23DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c23dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23DAR)

For information about available fields see [`mod@c23dar`] module*/
pub type C23DAR = crate::Reg<c23dar::C23DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c23dar;
/**C23BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c23brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23BRUR)

For information about available fields see [`mod@c23brur`] module*/
pub type C23BRUR = crate::Reg<c23brur::C23BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c23brur;
/**C23LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c23lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23LAR)

For information about available fields see [`mod@c23lar`] module*/
pub type C23LAR = crate::Reg<c23lar::C23LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c23lar;
/**C23TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c23tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23TBR)

For information about available fields see [`mod@c23tbr`] module*/
pub type C23TBR = crate::Reg<c23tbr::C23TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c23tbr;
/**C23MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c23mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23MAR)

For information about available fields see [`mod@c23mar`] module*/
pub type C23MAR = crate::Reg<c23mar::C23MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c23mar;
/**C23MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c23mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c23mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C23MDR)

For information about available fields see [`mod@c23mdr`] module*/
pub type C23MDR = crate::Reg<c23mdr::C23MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c23mdr;
/**C24ISR (r) register accessor: MDMA channel 24 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c24isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24ISR)

For information about available fields see [`mod@c24isr`] module*/
pub type C24ISR = crate::Reg<c24isr::C24ISRrs>;
///MDMA channel 24 interrupt/status register
pub mod c24isr;
/**C24IFCR (w) register accessor: MDMA channel 24 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24IFCR)

For information about available fields see [`mod@c24ifcr`] module*/
pub type C24IFCR = crate::Reg<c24ifcr::C24IFCRrs>;
///MDMA channel 24 interrupt flag clear register
pub mod c24ifcr;
/**C24ESR (r) register accessor: MDMA channel 24 error status register

You can [`read`](crate::Reg::read) this register and get [`c24esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24ESR)

For information about available fields see [`mod@c24esr`] module*/
pub type C24ESR = crate::Reg<c24esr::C24ESRrs>;
///MDMA channel 24 error status register
pub mod c24esr;
/**C24CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c24cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24CR)

For information about available fields see [`mod@c24cr`] module*/
pub type C24CR = crate::Reg<c24cr::C24CRrs>;
///This register is used to control the concerned channel.
pub mod c24cr;
/**C24TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c24tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24TCR)

For information about available fields see [`mod@c24tcr`] module*/
pub type C24TCR = crate::Reg<c24tcr::C24TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c24tcr;
/**C24BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c24bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24BNDTR)

For information about available fields see [`mod@c24bndtr`] module*/
pub type C24BNDTR = crate::Reg<c24bndtr::C24BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c24bndtr;
/**C24SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c24sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24SAR)

For information about available fields see [`mod@c24sar`] module*/
pub type C24SAR = crate::Reg<c24sar::C24SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c24sar;
/**C24DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c24dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24DAR)

For information about available fields see [`mod@c24dar`] module*/
pub type C24DAR = crate::Reg<c24dar::C24DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c24dar;
/**C24BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c24brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24BRUR)

For information about available fields see [`mod@c24brur`] module*/
pub type C24BRUR = crate::Reg<c24brur::C24BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c24brur;
/**C24LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c24lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24LAR)

For information about available fields see [`mod@c24lar`] module*/
pub type C24LAR = crate::Reg<c24lar::C24LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c24lar;
/**C24TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c24tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24TBR)

For information about available fields see [`mod@c24tbr`] module*/
pub type C24TBR = crate::Reg<c24tbr::C24TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c24tbr;
/**C24MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c24mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24MAR)

For information about available fields see [`mod@c24mar`] module*/
pub type C24MAR = crate::Reg<c24mar::C24MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c24mar;
/**C24MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c24mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c24mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C24MDR)

For information about available fields see [`mod@c24mdr`] module*/
pub type C24MDR = crate::Reg<c24mdr::C24MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c24mdr;
/**C25ISR (r) register accessor: MDMA channel 25 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c25isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25ISR)

For information about available fields see [`mod@c25isr`] module*/
pub type C25ISR = crate::Reg<c25isr::C25ISRrs>;
///MDMA channel 25 interrupt/status register
pub mod c25isr;
/**C25IFCR (w) register accessor: MDMA channel 25 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25IFCR)

For information about available fields see [`mod@c25ifcr`] module*/
pub type C25IFCR = crate::Reg<c25ifcr::C25IFCRrs>;
///MDMA channel 25 interrupt flag clear register
pub mod c25ifcr;
/**C25ESR (r) register accessor: MDMA channel 25 error status register

You can [`read`](crate::Reg::read) this register and get [`c25esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25ESR)

For information about available fields see [`mod@c25esr`] module*/
pub type C25ESR = crate::Reg<c25esr::C25ESRrs>;
///MDMA channel 25 error status register
pub mod c25esr;
/**C25CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c25cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25CR)

For information about available fields see [`mod@c25cr`] module*/
pub type C25CR = crate::Reg<c25cr::C25CRrs>;
///This register is used to control the concerned channel.
pub mod c25cr;
/**C25TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c25tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25TCR)

For information about available fields see [`mod@c25tcr`] module*/
pub type C25TCR = crate::Reg<c25tcr::C25TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c25tcr;
/**C25BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c25bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25BNDTR)

For information about available fields see [`mod@c25bndtr`] module*/
pub type C25BNDTR = crate::Reg<c25bndtr::C25BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c25bndtr;
/**C25SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c25sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25SAR)

For information about available fields see [`mod@c25sar`] module*/
pub type C25SAR = crate::Reg<c25sar::C25SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c25sar;
/**C25DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c25dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25DAR)

For information about available fields see [`mod@c25dar`] module*/
pub type C25DAR = crate::Reg<c25dar::C25DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c25dar;
/**C25BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c25brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25BRUR)

For information about available fields see [`mod@c25brur`] module*/
pub type C25BRUR = crate::Reg<c25brur::C25BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c25brur;
/**C25LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c25lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25LAR)

For information about available fields see [`mod@c25lar`] module*/
pub type C25LAR = crate::Reg<c25lar::C25LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c25lar;
/**C25TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c25tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25TBR)

For information about available fields see [`mod@c25tbr`] module*/
pub type C25TBR = crate::Reg<c25tbr::C25TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c25tbr;
/**C25MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c25mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25MAR)

For information about available fields see [`mod@c25mar`] module*/
pub type C25MAR = crate::Reg<c25mar::C25MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c25mar;
/**C25MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c25mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c25mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C25MDR)

For information about available fields see [`mod@c25mdr`] module*/
pub type C25MDR = crate::Reg<c25mdr::C25MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c25mdr;
/**C26ISR (r) register accessor: MDMA channel 26 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c26isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26ISR)

For information about available fields see [`mod@c26isr`] module*/
pub type C26ISR = crate::Reg<c26isr::C26ISRrs>;
///MDMA channel 26 interrupt/status register
pub mod c26isr;
/**C26IFCR (w) register accessor: MDMA channel 26 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26IFCR)

For information about available fields see [`mod@c26ifcr`] module*/
pub type C26IFCR = crate::Reg<c26ifcr::C26IFCRrs>;
///MDMA channel 26 interrupt flag clear register
pub mod c26ifcr;
/**C26ESR (r) register accessor: MDMA channel 26 error status register

You can [`read`](crate::Reg::read) this register and get [`c26esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26ESR)

For information about available fields see [`mod@c26esr`] module*/
pub type C26ESR = crate::Reg<c26esr::C26ESRrs>;
///MDMA channel 26 error status register
pub mod c26esr;
/**C26CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c26cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26CR)

For information about available fields see [`mod@c26cr`] module*/
pub type C26CR = crate::Reg<c26cr::C26CRrs>;
///This register is used to control the concerned channel.
pub mod c26cr;
/**C26TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c26tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26TCR)

For information about available fields see [`mod@c26tcr`] module*/
pub type C26TCR = crate::Reg<c26tcr::C26TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c26tcr;
/**C26BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c26bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26BNDTR)

For information about available fields see [`mod@c26bndtr`] module*/
pub type C26BNDTR = crate::Reg<c26bndtr::C26BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c26bndtr;
/**C26SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c26sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26SAR)

For information about available fields see [`mod@c26sar`] module*/
pub type C26SAR = crate::Reg<c26sar::C26SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c26sar;
/**C26DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c26dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26DAR)

For information about available fields see [`mod@c26dar`] module*/
pub type C26DAR = crate::Reg<c26dar::C26DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c26dar;
/**C26BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c26brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26BRUR)

For information about available fields see [`mod@c26brur`] module*/
pub type C26BRUR = crate::Reg<c26brur::C26BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c26brur;
/**C26LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c26lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26LAR)

For information about available fields see [`mod@c26lar`] module*/
pub type C26LAR = crate::Reg<c26lar::C26LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c26lar;
/**C26TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c26tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26TBR)

For information about available fields see [`mod@c26tbr`] module*/
pub type C26TBR = crate::Reg<c26tbr::C26TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c26tbr;
/**C26MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c26mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26MAR)

For information about available fields see [`mod@c26mar`] module*/
pub type C26MAR = crate::Reg<c26mar::C26MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c26mar;
/**C26MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c26mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c26mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C26MDR)

For information about available fields see [`mod@c26mdr`] module*/
pub type C26MDR = crate::Reg<c26mdr::C26MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c26mdr;
/**C27ISR (r) register accessor: MDMA channel 27 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c27isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27ISR)

For information about available fields see [`mod@c27isr`] module*/
pub type C27ISR = crate::Reg<c27isr::C27ISRrs>;
///MDMA channel 27 interrupt/status register
pub mod c27isr;
/**C27IFCR (w) register accessor: MDMA channel 27 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27IFCR)

For information about available fields see [`mod@c27ifcr`] module*/
pub type C27IFCR = crate::Reg<c27ifcr::C27IFCRrs>;
///MDMA channel 27 interrupt flag clear register
pub mod c27ifcr;
/**C27ESR (r) register accessor: MDMA channel 27 error status register

You can [`read`](crate::Reg::read) this register and get [`c27esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27ESR)

For information about available fields see [`mod@c27esr`] module*/
pub type C27ESR = crate::Reg<c27esr::C27ESRrs>;
///MDMA channel 27 error status register
pub mod c27esr;
/**C27CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c27cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27CR)

For information about available fields see [`mod@c27cr`] module*/
pub type C27CR = crate::Reg<c27cr::C27CRrs>;
///This register is used to control the concerned channel.
pub mod c27cr;
/**C27TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c27tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27TCR)

For information about available fields see [`mod@c27tcr`] module*/
pub type C27TCR = crate::Reg<c27tcr::C27TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c27tcr;
/**C27BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c27bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27BNDTR)

For information about available fields see [`mod@c27bndtr`] module*/
pub type C27BNDTR = crate::Reg<c27bndtr::C27BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c27bndtr;
/**C27SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c27sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27SAR)

For information about available fields see [`mod@c27sar`] module*/
pub type C27SAR = crate::Reg<c27sar::C27SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c27sar;
/**C27DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c27dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27DAR)

For information about available fields see [`mod@c27dar`] module*/
pub type C27DAR = crate::Reg<c27dar::C27DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c27dar;
/**C27BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c27brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27BRUR)

For information about available fields see [`mod@c27brur`] module*/
pub type C27BRUR = crate::Reg<c27brur::C27BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c27brur;
/**C27LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c27lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27LAR)

For information about available fields see [`mod@c27lar`] module*/
pub type C27LAR = crate::Reg<c27lar::C27LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c27lar;
/**C27TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c27tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27TBR)

For information about available fields see [`mod@c27tbr`] module*/
pub type C27TBR = crate::Reg<c27tbr::C27TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c27tbr;
/**C27MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c27mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27MAR)

For information about available fields see [`mod@c27mar`] module*/
pub type C27MAR = crate::Reg<c27mar::C27MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c27mar;
/**C27MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c27mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c27mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C27MDR)

For information about available fields see [`mod@c27mdr`] module*/
pub type C27MDR = crate::Reg<c27mdr::C27MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c27mdr;
/**C28ISR (r) register accessor: MDMA channel 28 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c28isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28ISR)

For information about available fields see [`mod@c28isr`] module*/
pub type C28ISR = crate::Reg<c28isr::C28ISRrs>;
///MDMA channel 28 interrupt/status register
pub mod c28isr;
/**C28IFCR (w) register accessor: MDMA channel 28 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28IFCR)

For information about available fields see [`mod@c28ifcr`] module*/
pub type C28IFCR = crate::Reg<c28ifcr::C28IFCRrs>;
///MDMA channel 28 interrupt flag clear register
pub mod c28ifcr;
/**C28ESR (r) register accessor: MDMA channel 28 error status register

You can [`read`](crate::Reg::read) this register and get [`c28esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28ESR)

For information about available fields see [`mod@c28esr`] module*/
pub type C28ESR = crate::Reg<c28esr::C28ESRrs>;
///MDMA channel 28 error status register
pub mod c28esr;
/**C28CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c28cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28CR)

For information about available fields see [`mod@c28cr`] module*/
pub type C28CR = crate::Reg<c28cr::C28CRrs>;
///This register is used to control the concerned channel.
pub mod c28cr;
/**C28TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c28tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28TCR)

For information about available fields see [`mod@c28tcr`] module*/
pub type C28TCR = crate::Reg<c28tcr::C28TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c28tcr;
/**C28BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c28bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28BNDTR)

For information about available fields see [`mod@c28bndtr`] module*/
pub type C28BNDTR = crate::Reg<c28bndtr::C28BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c28bndtr;
/**C28SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c28sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28SAR)

For information about available fields see [`mod@c28sar`] module*/
pub type C28SAR = crate::Reg<c28sar::C28SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c28sar;
/**C28DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c28dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28DAR)

For information about available fields see [`mod@c28dar`] module*/
pub type C28DAR = crate::Reg<c28dar::C28DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c28dar;
/**C28BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c28brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28BRUR)

For information about available fields see [`mod@c28brur`] module*/
pub type C28BRUR = crate::Reg<c28brur::C28BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c28brur;
/**C28LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c28lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28LAR)

For information about available fields see [`mod@c28lar`] module*/
pub type C28LAR = crate::Reg<c28lar::C28LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c28lar;
/**C28TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c28tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28TBR)

For information about available fields see [`mod@c28tbr`] module*/
pub type C28TBR = crate::Reg<c28tbr::C28TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c28tbr;
/**C28MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c28mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28MAR)

For information about available fields see [`mod@c28mar`] module*/
pub type C28MAR = crate::Reg<c28mar::C28MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c28mar;
/**C28MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c28mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c28mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C28MDR)

For information about available fields see [`mod@c28mdr`] module*/
pub type C28MDR = crate::Reg<c28mdr::C28MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c28mdr;
/**C29ISR (r) register accessor: MDMA channel 29 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c29isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29ISR)

For information about available fields see [`mod@c29isr`] module*/
pub type C29ISR = crate::Reg<c29isr::C29ISRrs>;
///MDMA channel 29 interrupt/status register
pub mod c29isr;
/**C29IFCR (w) register accessor: MDMA channel 29 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29IFCR)

For information about available fields see [`mod@c29ifcr`] module*/
pub type C29IFCR = crate::Reg<c29ifcr::C29IFCRrs>;
///MDMA channel 29 interrupt flag clear register
pub mod c29ifcr;
/**C29ESR (r) register accessor: MDMA channel 29 error status register

You can [`read`](crate::Reg::read) this register and get [`c29esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29ESR)

For information about available fields see [`mod@c29esr`] module*/
pub type C29ESR = crate::Reg<c29esr::C29ESRrs>;
///MDMA channel 29 error status register
pub mod c29esr;
/**C29CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c29cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29CR)

For information about available fields see [`mod@c29cr`] module*/
pub type C29CR = crate::Reg<c29cr::C29CRrs>;
///This register is used to control the concerned channel.
pub mod c29cr;
/**C29TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c29tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29TCR)

For information about available fields see [`mod@c29tcr`] module*/
pub type C29TCR = crate::Reg<c29tcr::C29TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c29tcr;
/**C29BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c29bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29BNDTR)

For information about available fields see [`mod@c29bndtr`] module*/
pub type C29BNDTR = crate::Reg<c29bndtr::C29BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c29bndtr;
/**C29SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c29sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29SAR)

For information about available fields see [`mod@c29sar`] module*/
pub type C29SAR = crate::Reg<c29sar::C29SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c29sar;
/**C29DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c29dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29DAR)

For information about available fields see [`mod@c29dar`] module*/
pub type C29DAR = crate::Reg<c29dar::C29DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c29dar;
/**C29BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c29brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29BRUR)

For information about available fields see [`mod@c29brur`] module*/
pub type C29BRUR = crate::Reg<c29brur::C29BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c29brur;
/**C29LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c29lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29LAR)

For information about available fields see [`mod@c29lar`] module*/
pub type C29LAR = crate::Reg<c29lar::C29LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c29lar;
/**C29TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c29tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29TBR)

For information about available fields see [`mod@c29tbr`] module*/
pub type C29TBR = crate::Reg<c29tbr::C29TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c29tbr;
/**C29MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c29mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29MAR)

For information about available fields see [`mod@c29mar`] module*/
pub type C29MAR = crate::Reg<c29mar::C29MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c29mar;
/**C29MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c29mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c29mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C29MDR)

For information about available fields see [`mod@c29mdr`] module*/
pub type C29MDR = crate::Reg<c29mdr::C29MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c29mdr;
/**C30ISR (r) register accessor: MDMA channel 30 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c30isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30ISR)

For information about available fields see [`mod@c30isr`] module*/
pub type C30ISR = crate::Reg<c30isr::C30ISRrs>;
///MDMA channel 30 interrupt/status register
pub mod c30isr;
/**C30IFCR (w) register accessor: MDMA channel 30 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30IFCR)

For information about available fields see [`mod@c30ifcr`] module*/
pub type C30IFCR = crate::Reg<c30ifcr::C30IFCRrs>;
///MDMA channel 30 interrupt flag clear register
pub mod c30ifcr;
/**C30ESR (r) register accessor: MDMA channel 30 error status register

You can [`read`](crate::Reg::read) this register and get [`c30esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30ESR)

For information about available fields see [`mod@c30esr`] module*/
pub type C30ESR = crate::Reg<c30esr::C30ESRrs>;
///MDMA channel 30 error status register
pub mod c30esr;
/**C30CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c30cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30CR)

For information about available fields see [`mod@c30cr`] module*/
pub type C30CR = crate::Reg<c30cr::C30CRrs>;
///This register is used to control the concerned channel.
pub mod c30cr;
/**C30TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c30tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30TCR)

For information about available fields see [`mod@c30tcr`] module*/
pub type C30TCR = crate::Reg<c30tcr::C30TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c30tcr;
/**C30BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c30bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30BNDTR)

For information about available fields see [`mod@c30bndtr`] module*/
pub type C30BNDTR = crate::Reg<c30bndtr::C30BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c30bndtr;
/**C30SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c30sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30SAR)

For information about available fields see [`mod@c30sar`] module*/
pub type C30SAR = crate::Reg<c30sar::C30SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c30sar;
/**C30DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c30dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30DAR)

For information about available fields see [`mod@c30dar`] module*/
pub type C30DAR = crate::Reg<c30dar::C30DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c30dar;
/**C30BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c30brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30BRUR)

For information about available fields see [`mod@c30brur`] module*/
pub type C30BRUR = crate::Reg<c30brur::C30BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c30brur;
/**C30LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c30lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30LAR)

For information about available fields see [`mod@c30lar`] module*/
pub type C30LAR = crate::Reg<c30lar::C30LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c30lar;
/**C30TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c30tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30TBR)

For information about available fields see [`mod@c30tbr`] module*/
pub type C30TBR = crate::Reg<c30tbr::C30TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c30tbr;
/**C30MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c30mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30MAR)

For information about available fields see [`mod@c30mar`] module*/
pub type C30MAR = crate::Reg<c30mar::C30MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c30mar;
/**C30MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c30mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c30mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C30MDR)

For information about available fields see [`mod@c30mdr`] module*/
pub type C30MDR = crate::Reg<c30mdr::C30MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c30mdr;
/**C31ISR (r) register accessor: MDMA channel 31 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`c31isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31ISR)

For information about available fields see [`mod@c31isr`] module*/
pub type C31ISR = crate::Reg<c31isr::C31ISRrs>;
///MDMA channel 31 interrupt/status register
pub mod c31isr;
/**C31IFCR (w) register accessor: MDMA channel 31 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31IFCR)

For information about available fields see [`mod@c31ifcr`] module*/
pub type C31IFCR = crate::Reg<c31ifcr::C31IFCRrs>;
///MDMA channel 31 interrupt flag clear register
pub mod c31ifcr;
/**C31ESR (r) register accessor: MDMA channel 31 error status register

You can [`read`](crate::Reg::read) this register and get [`c31esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31ESR)

For information about available fields see [`mod@c31esr`] module*/
pub type C31ESR = crate::Reg<c31esr::C31ESRrs>;
///MDMA channel 31 error status register
pub mod c31esr;
/**C31CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`c31cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31CR)

For information about available fields see [`mod@c31cr`] module*/
pub type C31CR = crate::Reg<c31cr::C31CRrs>;
///This register is used to control the concerned channel.
pub mod c31cr;
/**C31TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).

You can [`read`](crate::Reg::read) this register and get [`c31tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31TCR)

For information about available fields see [`mod@c31tcr`] module*/
pub type C31TCR = crate::Reg<c31tcr::C31TCRrs>;
///This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x00).
pub mod c31tcr;
/**C31BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).

You can [`read`](crate::Reg::read) this register and get [`c31bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31BNDTR)

For information about available fields see [`mod@c31bndtr`] module*/
pub type C31BNDTR = crate::Reg<c31bndtr::C31BNDTRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x04).
pub mod c31bndtr;
/**C31SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).

You can [`read`](crate::Reg::read) this register and get [`c31sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31SAR)

For information about available fields see [`mod@c31sar`] module*/
pub type C31SAR = crate::Reg<c31sar::C31SARrs>;
///In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x08).
pub mod c31sar;
/**C31DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`c31dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31DAR)

For information about available fields see [`mod@c31dar`] module*/
pub type C31DAR = crate::Reg<c31dar::C31DARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x0C). M
pub mod c31dar;
/**C31BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).

You can [`read`](crate::Reg::read) this register and get [`c31brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31BRUR)

For information about available fields see [`mod@c31brur`] module*/
pub type C31BRUR = crate::Reg<c31brur::C31BRURrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x10).
pub mod c31brur;
/**C31LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`c31lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31LAR)

For information about available fields see [`mod@c31lar`] module*/
pub type C31LAR = crate::Reg<c31lar::C31LARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.
pub mod c31lar;
/**C31TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).

You can [`read`](crate::Reg::read) this register and get [`c31tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31TBR)

For information about available fields see [`mod@c31tbr`] module*/
pub type C31TBR = crate::Reg<c31tbr::C31TBRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x18).
pub mod c31tbr;
/**C31MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).

You can [`read`](crate::Reg::read) this register and get [`c31mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31MAR)

For information about available fields see [`mod@c31mar`] module*/
pub type C31MAR = crate::Reg<c31mar::C31MARrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x20).
pub mod c31mar;
/**C31MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).

You can [`read`](crate::Reg::read) this register and get [`c31mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c31mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#MDMA:C31MDR)

For information about available fields see [`mod@c31mdr`] module*/
pub type C31MDR = crate::Reg<c31mdr::C31MDRrs>;
///In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\] + 0x24).
pub mod c31mdr;
