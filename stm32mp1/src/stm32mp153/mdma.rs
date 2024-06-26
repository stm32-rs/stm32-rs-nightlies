#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mdma_gisr0: MDMA_GISR0,
    _reserved1: [u8; 0x04],
    mdma_sgisr0: MDMA_SGISR0,
    _reserved2: [u8; 0x34],
    mdma_c0isr: MDMA_C0ISR,
    mdma_c0ifcr: MDMA_C0IFCR,
    mdma_c0esr: MDMA_C0ESR,
    mdma_c0cr: MDMA_C0CR,
    mdma_c0tcr: MDMA_C0TCR,
    mdma_c0bndtr: MDMA_C0BNDTR,
    mdma_c0sar: MDMA_C0SAR,
    mdma_c0dar: MDMA_C0DAR,
    mdma_c0brur: MDMA_C0BRUR,
    mdma_c0lar: MDMA_C0LAR,
    mdma_c0tbr: MDMA_C0TBR,
    _reserved13: [u8; 0x04],
    mdma_c0mar: MDMA_C0MAR,
    mdma_c0mdr: MDMA_C0MDR,
    _reserved15: [u8; 0x08],
    mdma_c1isr: MDMA_C1ISR,
    mdma_c1ifcr: MDMA_C1IFCR,
    mdma_c1esr: MDMA_C1ESR,
    mdma_c1cr: MDMA_C1CR,
    mdma_c1tcr: MDMA_C1TCR,
    mdma_c1bndtr: MDMA_C1BNDTR,
    mdma_c1sar: MDMA_C1SAR,
    mdma_c1dar: MDMA_C1DAR,
    mdma_c1brur: MDMA_C1BRUR,
    mdma_c1lar: MDMA_C1LAR,
    mdma_c1tbr: MDMA_C1TBR,
    _reserved26: [u8; 0x04],
    mdma_c1mar: MDMA_C1MAR,
    mdma_c1mdr: MDMA_C1MDR,
    _reserved28: [u8; 0x08],
    mdma_c2isr: MDMA_C2ISR,
    mdma_c2ifcr: MDMA_C2IFCR,
    mdma_c2esr: MDMA_C2ESR,
    mdma_c2cr: MDMA_C2CR,
    mdma_c2tcr: MDMA_C2TCR,
    mdma_c2bndtr: MDMA_C2BNDTR,
    mdma_c2sar: MDMA_C2SAR,
    mdma_c2dar: MDMA_C2DAR,
    mdma_c2brur: MDMA_C2BRUR,
    mdma_c2lar: MDMA_C2LAR,
    mdma_c2tbr: MDMA_C2TBR,
    _reserved39: [u8; 0x04],
    mdma_c2mar: MDMA_C2MAR,
    mdma_c2mdr: MDMA_C2MDR,
    _reserved41: [u8; 0x08],
    mdma_c3isr: MDMA_C3ISR,
    mdma_c3ifcr: MDMA_C3IFCR,
    mdma_c3esr: MDMA_C3ESR,
    mdma_c3cr: MDMA_C3CR,
    mdma_c3tcr: MDMA_C3TCR,
    mdma_c3bndtr: MDMA_C3BNDTR,
    mdma_c3sar: MDMA_C3SAR,
    mdma_c3dar: MDMA_C3DAR,
    mdma_c3brur: MDMA_C3BRUR,
    mdma_c3lar: MDMA_C3LAR,
    mdma_c3tbr: MDMA_C3TBR,
    _reserved52: [u8; 0x04],
    mdma_c3mar: MDMA_C3MAR,
    mdma_c3mdr: MDMA_C3MDR,
    _reserved54: [u8; 0x08],
    mdma_c4isr: MDMA_C4ISR,
    mdma_c4ifcr: MDMA_C4IFCR,
    mdma_c4esr: MDMA_C4ESR,
    mdma_c4cr: MDMA_C4CR,
    mdma_c4tcr: MDMA_C4TCR,
    mdma_c4bndtr: MDMA_C4BNDTR,
    mdma_c4sar: MDMA_C4SAR,
    mdma_c4dar: MDMA_C4DAR,
    mdma_c4brur: MDMA_C4BRUR,
    mdma_c4lar: MDMA_C4LAR,
    mdma_c4tbr: MDMA_C4TBR,
    _reserved65: [u8; 0x04],
    mdma_c4mar: MDMA_C4MAR,
    mdma_c4mdr: MDMA_C4MDR,
    _reserved67: [u8; 0x08],
    mdma_c5isr: MDMA_C5ISR,
    mdma_c5ifcr: MDMA_C5IFCR,
    mdma_c5esr: MDMA_C5ESR,
    mdma_c5cr: MDMA_C5CR,
    mdma_c5tcr: MDMA_C5TCR,
    mdma_c5bndtr: MDMA_C5BNDTR,
    mdma_c5sar: MDMA_C5SAR,
    mdma_c5dar: MDMA_C5DAR,
    mdma_c5brur: MDMA_C5BRUR,
    mdma_c5lar: MDMA_C5LAR,
    mdma_c5tbr: MDMA_C5TBR,
    _reserved78: [u8; 0x04],
    mdma_c5mar: MDMA_C5MAR,
    mdma_c5mdr: MDMA_C5MDR,
    _reserved80: [u8; 0x08],
    mdma_c6isr: MDMA_C6ISR,
    mdma_c6ifcr: MDMA_C6IFCR,
    mdma_c6esr: MDMA_C6ESR,
    mdma_c6cr: MDMA_C6CR,
    mdma_c6tcr: MDMA_C6TCR,
    mdma_c6bndtr: MDMA_C6BNDTR,
    mdma_c6sar: MDMA_C6SAR,
    mdma_c6dar: MDMA_C6DAR,
    mdma_c6brur: MDMA_C6BRUR,
    mdma_c6lar: MDMA_C6LAR,
    mdma_c6tbr: MDMA_C6TBR,
    _reserved91: [u8; 0x04],
    mdma_c6mar: MDMA_C6MAR,
    mdma_c6mdr: MDMA_C6MDR,
    _reserved93: [u8; 0x08],
    mdma_c7isr: MDMA_C7ISR,
    mdma_c7ifcr: MDMA_C7IFCR,
    mdma_c7esr: MDMA_C7ESR,
    mdma_c7cr: MDMA_C7CR,
    mdma_c7tcr: MDMA_C7TCR,
    mdma_c7bndtr: MDMA_C7BNDTR,
    mdma_c7sar: MDMA_C7SAR,
    mdma_c7dar: MDMA_C7DAR,
    mdma_c7brur: MDMA_C7BRUR,
    mdma_c7lar: MDMA_C7LAR,
    mdma_c7tbr: MDMA_C7TBR,
    _reserved104: [u8; 0x04],
    mdma_c7mar: MDMA_C7MAR,
    mdma_c7mdr: MDMA_C7MDR,
    _reserved106: [u8; 0x08],
    mdma_c8isr: MDMA_C8ISR,
    mdma_c8ifcr: MDMA_C8IFCR,
    mdma_c8esr: MDMA_C8ESR,
    mdma_c8cr: MDMA_C8CR,
    mdma_c8tcr: MDMA_C8TCR,
    mdma_c8bndtr: MDMA_C8BNDTR,
    mdma_c8sar: MDMA_C8SAR,
    mdma_c8dar: MDMA_C8DAR,
    mdma_c8brur: MDMA_C8BRUR,
    mdma_c8lar: MDMA_C8LAR,
    mdma_c8tbr: MDMA_C8TBR,
    _reserved117: [u8; 0x04],
    mdma_c8mar: MDMA_C8MAR,
    mdma_c8mdr: MDMA_C8MDR,
    _reserved119: [u8; 0x08],
    mdma_c9isr: MDMA_C9ISR,
    mdma_c9ifcr: MDMA_C9IFCR,
    mdma_c9esr: MDMA_C9ESR,
    mdma_c9cr: MDMA_C9CR,
    mdma_c9tcr: MDMA_C9TCR,
    mdma_c9bndtr: MDMA_C9BNDTR,
    mdma_c9sar: MDMA_C9SAR,
    mdma_c9dar: MDMA_C9DAR,
    mdma_c9brur: MDMA_C9BRUR,
    mdma_c9lar: MDMA_C9LAR,
    mdma_c9tbr: MDMA_C9TBR,
    _reserved130: [u8; 0x04],
    mdma_c9mar: MDMA_C9MAR,
    mdma_c9mdr: MDMA_C9MDR,
    _reserved132: [u8; 0x08],
    mdma_c10isr: MDMA_C10ISR,
    mdma_c10ifcr: MDMA_C10IFCR,
    mdma_c10esr: MDMA_C10ESR,
    mdma_c10cr: MDMA_C10CR,
    mdma_c10tcr: MDMA_C10TCR,
    mdma_c10bndtr: MDMA_C10BNDTR,
    mdma_c10sar: MDMA_C10SAR,
    mdma_c10dar: MDMA_C10DAR,
    mdma_c10brur: MDMA_C10BRUR,
    mdma_c10lar: MDMA_C10LAR,
    mdma_c10tbr: MDMA_C10TBR,
    _reserved143: [u8; 0x04],
    mdma_c10mar: MDMA_C10MAR,
    mdma_c10mdr: MDMA_C10MDR,
    _reserved145: [u8; 0x08],
    mdma_c11isr: MDMA_C11ISR,
    mdma_c11ifcr: MDMA_C11IFCR,
    mdma_c11esr: MDMA_C11ESR,
    mdma_c11cr: MDMA_C11CR,
    mdma_c11tcr: MDMA_C11TCR,
    mdma_c11bndtr: MDMA_C11BNDTR,
    mdma_c11sar: MDMA_C11SAR,
    mdma_c11dar: MDMA_C11DAR,
    mdma_c11brur: MDMA_C11BRUR,
    mdma_c11lar: MDMA_C11LAR,
    mdma_c11tbr: MDMA_C11TBR,
    _reserved156: [u8; 0x04],
    mdma_c11mar: MDMA_C11MAR,
    mdma_c11mdr: MDMA_C11MDR,
    _reserved158: [u8; 0x08],
    mdma_c12isr: MDMA_C12ISR,
    mdma_c12ifcr: MDMA_C12IFCR,
    mdma_c12esr: MDMA_C12ESR,
    mdma_c12cr: MDMA_C12CR,
    mdma_c12tcr: MDMA_C12TCR,
    mdma_c12bndtr: MDMA_C12BNDTR,
    mdma_c12sar: MDMA_C12SAR,
    mdma_c12dar: MDMA_C12DAR,
    mdma_c12brur: MDMA_C12BRUR,
    mdma_c12lar: MDMA_C12LAR,
    mdma_c12tbr: MDMA_C12TBR,
    _reserved169: [u8; 0x04],
    mdma_c12mar: MDMA_C12MAR,
    mdma_c12mdr: MDMA_C12MDR,
    _reserved171: [u8; 0x08],
    mdma_c13isr: MDMA_C13ISR,
    mdma_c13ifcr: MDMA_C13IFCR,
    mdma_c13esr: MDMA_C13ESR,
    mdma_c13cr: MDMA_C13CR,
    mdma_c13tcr: MDMA_C13TCR,
    mdma_c13bndtr: MDMA_C13BNDTR,
    mdma_c13sar: MDMA_C13SAR,
    mdma_c13dar: MDMA_C13DAR,
    mdma_c13brur: MDMA_C13BRUR,
    mdma_c13lar: MDMA_C13LAR,
    mdma_c13tbr: MDMA_C13TBR,
    _reserved182: [u8; 0x04],
    mdma_c13mar: MDMA_C13MAR,
    mdma_c13mdr: MDMA_C13MDR,
    _reserved184: [u8; 0x08],
    mdma_c14isr: MDMA_C14ISR,
    mdma_c14ifcr: MDMA_C14IFCR,
    mdma_c14esr: MDMA_C14ESR,
    mdma_c14cr: MDMA_C14CR,
    mdma_c14tcr: MDMA_C14TCR,
    mdma_c14bndtr: MDMA_C14BNDTR,
    mdma_c14sar: MDMA_C14SAR,
    mdma_c14dar: MDMA_C14DAR,
    mdma_c14brur: MDMA_C14BRUR,
    mdma_c14lar: MDMA_C14LAR,
    mdma_c14tbr: MDMA_C14TBR,
    _reserved195: [u8; 0x04],
    mdma_c14mar: MDMA_C14MAR,
    mdma_c14mdr: MDMA_C14MDR,
    _reserved197: [u8; 0x08],
    mdma_c15isr: MDMA_C15ISR,
    mdma_c15ifcr: MDMA_C15IFCR,
    mdma_c15esr: MDMA_C15ESR,
    mdma_c15cr: MDMA_C15CR,
    mdma_c15tcr: MDMA_C15TCR,
    mdma_c15bndtr: MDMA_C15BNDTR,
    mdma_c15sar: MDMA_C15SAR,
    mdma_c15dar: MDMA_C15DAR,
    mdma_c15brur: MDMA_C15BRUR,
    mdma_c15lar: MDMA_C15LAR,
    mdma_c15tbr: MDMA_C15TBR,
    _reserved208: [u8; 0x04],
    mdma_c15mar: MDMA_C15MAR,
    mdma_c15mdr: MDMA_C15MDR,
    _reserved210: [u8; 0x08],
    mdma_c16isr: MDMA_C16ISR,
    mdma_c16ifcr: MDMA_C16IFCR,
    mdma_c16esr: MDMA_C16ESR,
    mdma_c16cr: MDMA_C16CR,
    mdma_c16tcr: MDMA_C16TCR,
    mdma_c16bndtr: MDMA_C16BNDTR,
    mdma_c16sar: MDMA_C16SAR,
    mdma_c16dar: MDMA_C16DAR,
    mdma_c16brur: MDMA_C16BRUR,
    mdma_c16lar: MDMA_C16LAR,
    mdma_c16tbr: MDMA_C16TBR,
    _reserved221: [u8; 0x04],
    mdma_c16mar: MDMA_C16MAR,
    mdma_c16mdr: MDMA_C16MDR,
    _reserved223: [u8; 0x08],
    mdma_c17isr: MDMA_C17ISR,
    mdma_c17ifcr: MDMA_C17IFCR,
    mdma_c17esr: MDMA_C17ESR,
    mdma_c17cr: MDMA_C17CR,
    mdma_c17tcr: MDMA_C17TCR,
    mdma_c17bndtr: MDMA_C17BNDTR,
    mdma_c17sar: MDMA_C17SAR,
    mdma_c17dar: MDMA_C17DAR,
    mdma_c17brur: MDMA_C17BRUR,
    mdma_c17lar: MDMA_C17LAR,
    mdma_c17tbr: MDMA_C17TBR,
    _reserved234: [u8; 0x04],
    mdma_c17mar: MDMA_C17MAR,
    mdma_c17mdr: MDMA_C17MDR,
    _reserved236: [u8; 0x08],
    mdma_c18isr: MDMA_C18ISR,
    mdma_c18ifcr: MDMA_C18IFCR,
    mdma_c18esr: MDMA_C18ESR,
    mdma_c18cr: MDMA_C18CR,
    mdma_c18tcr: MDMA_C18TCR,
    mdma_c18bndtr: MDMA_C18BNDTR,
    mdma_c18sar: MDMA_C18SAR,
    mdma_c18dar: MDMA_C18DAR,
    mdma_c18brur: MDMA_C18BRUR,
    mdma_c18lar: MDMA_C18LAR,
    mdma_c18tbr: MDMA_C18TBR,
    _reserved247: [u8; 0x04],
    mdma_c18mar: MDMA_C18MAR,
    mdma_c18mdr: MDMA_C18MDR,
    _reserved249: [u8; 0x08],
    mdma_c19isr: MDMA_C19ISR,
    mdma_c19ifcr: MDMA_C19IFCR,
    mdma_c19esr: MDMA_C19ESR,
    mdma_c19cr: MDMA_C19CR,
    mdma_c19tcr: MDMA_C19TCR,
    mdma_c19bndtr: MDMA_C19BNDTR,
    mdma_c19sar: MDMA_C19SAR,
    mdma_c19dar: MDMA_C19DAR,
    mdma_c19brur: MDMA_C19BRUR,
    mdma_c19lar: MDMA_C19LAR,
    mdma_c19tbr: MDMA_C19TBR,
    _reserved260: [u8; 0x04],
    mdma_c19mar: MDMA_C19MAR,
    mdma_c19mdr: MDMA_C19MDR,
    _reserved262: [u8; 0x08],
    mdma_c20isr: MDMA_C20ISR,
    mdma_c20ifcr: MDMA_C20IFCR,
    mdma_c20esr: MDMA_C20ESR,
    mdma_c20cr: MDMA_C20CR,
    mdma_c20tcr: MDMA_C20TCR,
    mdma_c20bndtr: MDMA_C20BNDTR,
    mdma_c20sar: MDMA_C20SAR,
    mdma_c20dar: MDMA_C20DAR,
    mdma_c20brur: MDMA_C20BRUR,
    mdma_c20lar: MDMA_C20LAR,
    mdma_c20tbr: MDMA_C20TBR,
    _reserved273: [u8; 0x04],
    mdma_c20mar: MDMA_C20MAR,
    mdma_c20mdr: MDMA_C20MDR,
    _reserved275: [u8; 0x08],
    mdma_c21isr: MDMA_C21ISR,
    mdma_c21ifcr: MDMA_C21IFCR,
    mdma_c21esr: MDMA_C21ESR,
    mdma_c21cr: MDMA_C21CR,
    mdma_c21tcr: MDMA_C21TCR,
    mdma_c21bndtr: MDMA_C21BNDTR,
    mdma_c21sar: MDMA_C21SAR,
    mdma_c21dar: MDMA_C21DAR,
    mdma_c21brur: MDMA_C21BRUR,
    mdma_c21lar: MDMA_C21LAR,
    mdma_c21tbr: MDMA_C21TBR,
    _reserved286: [u8; 0x04],
    mdma_c21mar: MDMA_C21MAR,
    mdma_c21mdr: MDMA_C21MDR,
    _reserved288: [u8; 0x08],
    mdma_c22isr: MDMA_C22ISR,
    mdma_c22ifcr: MDMA_C22IFCR,
    mdma_c22esr: MDMA_C22ESR,
    mdma_c22cr: MDMA_C22CR,
    mdma_c22tcr: MDMA_C22TCR,
    mdma_c22bndtr: MDMA_C22BNDTR,
    mdma_c22sar: MDMA_C22SAR,
    mdma_c22dar: MDMA_C22DAR,
    mdma_c22brur: MDMA_C22BRUR,
    mdma_c22lar: MDMA_C22LAR,
    mdma_c22tbr: MDMA_C22TBR,
    _reserved299: [u8; 0x04],
    mdma_c22mar: MDMA_C22MAR,
    mdma_c22mdr: MDMA_C22MDR,
    _reserved301: [u8; 0x08],
    mdma_c23isr: MDMA_C23ISR,
    mdma_c23ifcr: MDMA_C23IFCR,
    mdma_c23esr: MDMA_C23ESR,
    mdma_c23cr: MDMA_C23CR,
    mdma_c23tcr: MDMA_C23TCR,
    mdma_c23bndtr: MDMA_C23BNDTR,
    mdma_c23sar: MDMA_C23SAR,
    mdma_c23dar: MDMA_C23DAR,
    mdma_c23brur: MDMA_C23BRUR,
    mdma_c23lar: MDMA_C23LAR,
    mdma_c23tbr: MDMA_C23TBR,
    _reserved312: [u8; 0x04],
    mdma_c23mar: MDMA_C23MAR,
    mdma_c23mdr: MDMA_C23MDR,
    _reserved314: [u8; 0x08],
    mdma_c24isr: MDMA_C24ISR,
    mdma_c24ifcr: MDMA_C24IFCR,
    mdma_c24esr: MDMA_C24ESR,
    mdma_c24cr: MDMA_C24CR,
    mdma_c24tcr: MDMA_C24TCR,
    mdma_c24bndtr: MDMA_C24BNDTR,
    mdma_c24sar: MDMA_C24SAR,
    mdma_c24dar: MDMA_C24DAR,
    mdma_c24brur: MDMA_C24BRUR,
    mdma_c24lar: MDMA_C24LAR,
    mdma_c24tbr: MDMA_C24TBR,
    _reserved325: [u8; 0x04],
    mdma_c24mar: MDMA_C24MAR,
    mdma_c24mdr: MDMA_C24MDR,
    _reserved327: [u8; 0x08],
    mdma_c25isr: MDMA_C25ISR,
    mdma_c25ifcr: MDMA_C25IFCR,
    mdma_c25esr: MDMA_C25ESR,
    mdma_c25cr: MDMA_C25CR,
    mdma_c25tcr: MDMA_C25TCR,
    mdma_c25bndtr: MDMA_C25BNDTR,
    mdma_c25sar: MDMA_C25SAR,
    mdma_c25dar: MDMA_C25DAR,
    mdma_c25brur: MDMA_C25BRUR,
    mdma_c25lar: MDMA_C25LAR,
    mdma_c25tbr: MDMA_C25TBR,
    _reserved338: [u8; 0x04],
    mdma_c25mar: MDMA_C25MAR,
    mdma_c25mdr: MDMA_C25MDR,
    _reserved340: [u8; 0x08],
    mdma_c26isr: MDMA_C26ISR,
    mdma_c26ifcr: MDMA_C26IFCR,
    mdma_c26esr: MDMA_C26ESR,
    mdma_c26cr: MDMA_C26CR,
    mdma_c26tcr: MDMA_C26TCR,
    mdma_c26bndtr: MDMA_C26BNDTR,
    mdma_c26sar: MDMA_C26SAR,
    mdma_c26dar: MDMA_C26DAR,
    mdma_c26brur: MDMA_C26BRUR,
    mdma_c26lar: MDMA_C26LAR,
    mdma_c26tbr: MDMA_C26TBR,
    _reserved351: [u8; 0x04],
    mdma_c26mar: MDMA_C26MAR,
    mdma_c26mdr: MDMA_C26MDR,
    _reserved353: [u8; 0x08],
    mdma_c27isr: MDMA_C27ISR,
    mdma_c27ifcr: MDMA_C27IFCR,
    mdma_c27esr: MDMA_C27ESR,
    mdma_c27cr: MDMA_C27CR,
    mdma_c27tcr: MDMA_C27TCR,
    mdma_c27bndtr: MDMA_C27BNDTR,
    mdma_c27sar: MDMA_C27SAR,
    mdma_c27dar: MDMA_C27DAR,
    mdma_c27brur: MDMA_C27BRUR,
    mdma_c27lar: MDMA_C27LAR,
    mdma_c27tbr: MDMA_C27TBR,
    _reserved364: [u8; 0x04],
    mdma_c27mar: MDMA_C27MAR,
    mdma_c27mdr: MDMA_C27MDR,
    _reserved366: [u8; 0x08],
    mdma_c28isr: MDMA_C28ISR,
    mdma_c28ifcr: MDMA_C28IFCR,
    mdma_c28esr: MDMA_C28ESR,
    mdma_c28cr: MDMA_C28CR,
    mdma_c28tcr: MDMA_C28TCR,
    mdma_c28bndtr: MDMA_C28BNDTR,
    mdma_c28sar: MDMA_C28SAR,
    mdma_c28dar: MDMA_C28DAR,
    mdma_c28brur: MDMA_C28BRUR,
    mdma_c28lar: MDMA_C28LAR,
    mdma_c28tbr: MDMA_C28TBR,
    _reserved377: [u8; 0x04],
    mdma_c28mar: MDMA_C28MAR,
    mdma_c28mdr: MDMA_C28MDR,
    _reserved379: [u8; 0x08],
    mdma_c29isr: MDMA_C29ISR,
    mdma_c29ifcr: MDMA_C29IFCR,
    mdma_c29esr: MDMA_C29ESR,
    mdma_c29cr: MDMA_C29CR,
    mdma_c29tcr: MDMA_C29TCR,
    mdma_c29bndtr: MDMA_C29BNDTR,
    mdma_c29sar: MDMA_C29SAR,
    mdma_c29dar: MDMA_C29DAR,
    mdma_c29brur: MDMA_C29BRUR,
    mdma_c29lar: MDMA_C29LAR,
    mdma_c29tbr: MDMA_C29TBR,
    _reserved390: [u8; 0x04],
    mdma_c29mar: MDMA_C29MAR,
    mdma_c29mdr: MDMA_C29MDR,
    _reserved392: [u8; 0x08],
    mdma_c30isr: MDMA_C30ISR,
    mdma_c30ifcr: MDMA_C30IFCR,
    mdma_c30esr: MDMA_C30ESR,
    mdma_c30cr: MDMA_C30CR,
    mdma_c30tcr: MDMA_C30TCR,
    mdma_c30bndtr: MDMA_C30BNDTR,
    mdma_c30sar: MDMA_C30SAR,
    mdma_c30dar: MDMA_C30DAR,
    mdma_c30brur: MDMA_C30BRUR,
    mdma_c30lar: MDMA_C30LAR,
    mdma_c30tbr: MDMA_C30TBR,
    _reserved403: [u8; 0x04],
    mdma_c30mar: MDMA_C30MAR,
    mdma_c30mdr: MDMA_C30MDR,
    _reserved405: [u8; 0x08],
    mdma_c31isr: MDMA_C31ISR,
    mdma_c31ifcr: MDMA_C31IFCR,
    mdma_c31esr: MDMA_C31ESR,
    mdma_c31cr: MDMA_C31CR,
    mdma_c31tcr: MDMA_C31TCR,
    mdma_c31bndtr: MDMA_C31BNDTR,
    mdma_c31sar: MDMA_C31SAR,
    mdma_c31dar: MDMA_C31DAR,
    mdma_c31brur: MDMA_C31BRUR,
    mdma_c31lar: MDMA_C31LAR,
    mdma_c31tbr: MDMA_C31TBR,
    _reserved416: [u8; 0x04],
    mdma_c31mar: MDMA_C31MAR,
    mdma_c31mdr: MDMA_C31MDR,
}
impl RegisterBlock {
    ///0x00 - MDMA global interrupt/status register
    #[inline(always)]
    pub const fn mdma_gisr0(&self) -> &MDMA_GISR0 {
        &self.mdma_gisr0
    }
    ///0x08 - MDMA secure global interrupt/status register
    #[inline(always)]
    pub const fn mdma_sgisr0(&self) -> &MDMA_SGISR0 {
        &self.mdma_sgisr0
    }
    ///0x40 - MDMA channel 0 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c0isr(&self) -> &MDMA_C0ISR {
        &self.mdma_c0isr
    }
    ///0x44 - MDMA channel 0 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c0ifcr(&self) -> &MDMA_C0IFCR {
        &self.mdma_c0ifcr
    }
    ///0x48 - MDMA channel 0 error status register
    #[inline(always)]
    pub const fn mdma_c0esr(&self) -> &MDMA_C0ESR {
        &self.mdma_c0esr
    }
    ///0x4c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c0cr(&self) -> &MDMA_C0CR {
        &self.mdma_c0cr
    }
    /**0x50 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c0tcr(&self) -> &MDMA_C0TCR {
        &self.mdma_c0tcr
    }
    /**0x54 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c0bndtr(&self) -> &MDMA_C0BNDTR {
        &self.mdma_c0bndtr
    }
    /**0x58 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c0sar(&self) -> &MDMA_C0SAR {
        &self.mdma_c0sar
    }
    /**0x5c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c0dar(&self) -> &MDMA_C0DAR {
        &self.mdma_c0dar
    }
    /**0x60 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c0brur(&self) -> &MDMA_C0BRUR {
        &self.mdma_c0brur
    }
    /**0x64 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c0lar(&self) -> &MDMA_C0LAR {
        &self.mdma_c0lar
    }
    /**0x68 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c0tbr(&self) -> &MDMA_C0TBR {
        &self.mdma_c0tbr
    }
    /**0x70 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c0mar(&self) -> &MDMA_C0MAR {
        &self.mdma_c0mar
    }
    /**0x74 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c0mdr(&self) -> &MDMA_C0MDR {
        &self.mdma_c0mdr
    }
    ///0x80 - MDMA channel 1 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c1isr(&self) -> &MDMA_C1ISR {
        &self.mdma_c1isr
    }
    ///0x84 - MDMA channel 1 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c1ifcr(&self) -> &MDMA_C1IFCR {
        &self.mdma_c1ifcr
    }
    ///0x88 - MDMA channel 1 error status register
    #[inline(always)]
    pub const fn mdma_c1esr(&self) -> &MDMA_C1ESR {
        &self.mdma_c1esr
    }
    ///0x8c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c1cr(&self) -> &MDMA_C1CR {
        &self.mdma_c1cr
    }
    /**0x90 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c1tcr(&self) -> &MDMA_C1TCR {
        &self.mdma_c1tcr
    }
    /**0x94 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c1bndtr(&self) -> &MDMA_C1BNDTR {
        &self.mdma_c1bndtr
    }
    /**0x98 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c1sar(&self) -> &MDMA_C1SAR {
        &self.mdma_c1sar
    }
    /**0x9c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c1dar(&self) -> &MDMA_C1DAR {
        &self.mdma_c1dar
    }
    /**0xa0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c1brur(&self) -> &MDMA_C1BRUR {
        &self.mdma_c1brur
    }
    /**0xa4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c1lar(&self) -> &MDMA_C1LAR {
        &self.mdma_c1lar
    }
    /**0xa8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c1tbr(&self) -> &MDMA_C1TBR {
        &self.mdma_c1tbr
    }
    /**0xb0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c1mar(&self) -> &MDMA_C1MAR {
        &self.mdma_c1mar
    }
    /**0xb4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c1mdr(&self) -> &MDMA_C1MDR {
        &self.mdma_c1mdr
    }
    ///0xc0 - MDMA channel 2 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c2isr(&self) -> &MDMA_C2ISR {
        &self.mdma_c2isr
    }
    ///0xc4 - MDMA channel 2 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c2ifcr(&self) -> &MDMA_C2IFCR {
        &self.mdma_c2ifcr
    }
    ///0xc8 - MDMA channel 2 error status register
    #[inline(always)]
    pub const fn mdma_c2esr(&self) -> &MDMA_C2ESR {
        &self.mdma_c2esr
    }
    ///0xcc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c2cr(&self) -> &MDMA_C2CR {
        &self.mdma_c2cr
    }
    /**0xd0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c2tcr(&self) -> &MDMA_C2TCR {
        &self.mdma_c2tcr
    }
    /**0xd4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c2bndtr(&self) -> &MDMA_C2BNDTR {
        &self.mdma_c2bndtr
    }
    /**0xd8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c2sar(&self) -> &MDMA_C2SAR {
        &self.mdma_c2sar
    }
    /**0xdc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c2dar(&self) -> &MDMA_C2DAR {
        &self.mdma_c2dar
    }
    /**0xe0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c2brur(&self) -> &MDMA_C2BRUR {
        &self.mdma_c2brur
    }
    /**0xe4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c2lar(&self) -> &MDMA_C2LAR {
        &self.mdma_c2lar
    }
    /**0xe8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c2tbr(&self) -> &MDMA_C2TBR {
        &self.mdma_c2tbr
    }
    /**0xf0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c2mar(&self) -> &MDMA_C2MAR {
        &self.mdma_c2mar
    }
    /**0xf4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c2mdr(&self) -> &MDMA_C2MDR {
        &self.mdma_c2mdr
    }
    ///0x100 - MDMA channel 3 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c3isr(&self) -> &MDMA_C3ISR {
        &self.mdma_c3isr
    }
    ///0x104 - MDMA channel 3 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c3ifcr(&self) -> &MDMA_C3IFCR {
        &self.mdma_c3ifcr
    }
    ///0x108 - MDMA channel 3 error status register
    #[inline(always)]
    pub const fn mdma_c3esr(&self) -> &MDMA_C3ESR {
        &self.mdma_c3esr
    }
    ///0x10c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c3cr(&self) -> &MDMA_C3CR {
        &self.mdma_c3cr
    }
    /**0x110 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c3tcr(&self) -> &MDMA_C3TCR {
        &self.mdma_c3tcr
    }
    /**0x114 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c3bndtr(&self) -> &MDMA_C3BNDTR {
        &self.mdma_c3bndtr
    }
    /**0x118 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c3sar(&self) -> &MDMA_C3SAR {
        &self.mdma_c3sar
    }
    /**0x11c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c3dar(&self) -> &MDMA_C3DAR {
        &self.mdma_c3dar
    }
    /**0x120 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c3brur(&self) -> &MDMA_C3BRUR {
        &self.mdma_c3brur
    }
    /**0x124 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c3lar(&self) -> &MDMA_C3LAR {
        &self.mdma_c3lar
    }
    /**0x128 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c3tbr(&self) -> &MDMA_C3TBR {
        &self.mdma_c3tbr
    }
    /**0x130 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c3mar(&self) -> &MDMA_C3MAR {
        &self.mdma_c3mar
    }
    /**0x134 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c3mdr(&self) -> &MDMA_C3MDR {
        &self.mdma_c3mdr
    }
    ///0x140 - MDMA channel 4 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c4isr(&self) -> &MDMA_C4ISR {
        &self.mdma_c4isr
    }
    ///0x144 - MDMA channel 4 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c4ifcr(&self) -> &MDMA_C4IFCR {
        &self.mdma_c4ifcr
    }
    ///0x148 - MDMA channel 4 error status register
    #[inline(always)]
    pub const fn mdma_c4esr(&self) -> &MDMA_C4ESR {
        &self.mdma_c4esr
    }
    ///0x14c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c4cr(&self) -> &MDMA_C4CR {
        &self.mdma_c4cr
    }
    /**0x150 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c4tcr(&self) -> &MDMA_C4TCR {
        &self.mdma_c4tcr
    }
    /**0x154 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c4bndtr(&self) -> &MDMA_C4BNDTR {
        &self.mdma_c4bndtr
    }
    /**0x158 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c4sar(&self) -> &MDMA_C4SAR {
        &self.mdma_c4sar
    }
    /**0x15c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c4dar(&self) -> &MDMA_C4DAR {
        &self.mdma_c4dar
    }
    /**0x160 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c4brur(&self) -> &MDMA_C4BRUR {
        &self.mdma_c4brur
    }
    /**0x164 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c4lar(&self) -> &MDMA_C4LAR {
        &self.mdma_c4lar
    }
    /**0x168 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c4tbr(&self) -> &MDMA_C4TBR {
        &self.mdma_c4tbr
    }
    /**0x170 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c4mar(&self) -> &MDMA_C4MAR {
        &self.mdma_c4mar
    }
    /**0x174 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c4mdr(&self) -> &MDMA_C4MDR {
        &self.mdma_c4mdr
    }
    ///0x180 - MDMA channel 5 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c5isr(&self) -> &MDMA_C5ISR {
        &self.mdma_c5isr
    }
    ///0x184 - MDMA channel 5 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c5ifcr(&self) -> &MDMA_C5IFCR {
        &self.mdma_c5ifcr
    }
    ///0x188 - MDMA channel 5 error status register
    #[inline(always)]
    pub const fn mdma_c5esr(&self) -> &MDMA_C5ESR {
        &self.mdma_c5esr
    }
    ///0x18c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c5cr(&self) -> &MDMA_C5CR {
        &self.mdma_c5cr
    }
    /**0x190 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c5tcr(&self) -> &MDMA_C5TCR {
        &self.mdma_c5tcr
    }
    /**0x194 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c5bndtr(&self) -> &MDMA_C5BNDTR {
        &self.mdma_c5bndtr
    }
    /**0x198 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c5sar(&self) -> &MDMA_C5SAR {
        &self.mdma_c5sar
    }
    /**0x19c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c5dar(&self) -> &MDMA_C5DAR {
        &self.mdma_c5dar
    }
    /**0x1a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c5brur(&self) -> &MDMA_C5BRUR {
        &self.mdma_c5brur
    }
    /**0x1a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c5lar(&self) -> &MDMA_C5LAR {
        &self.mdma_c5lar
    }
    /**0x1a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c5tbr(&self) -> &MDMA_C5TBR {
        &self.mdma_c5tbr
    }
    /**0x1b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c5mar(&self) -> &MDMA_C5MAR {
        &self.mdma_c5mar
    }
    /**0x1b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c5mdr(&self) -> &MDMA_C5MDR {
        &self.mdma_c5mdr
    }
    ///0x1c0 - MDMA channel 6 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c6isr(&self) -> &MDMA_C6ISR {
        &self.mdma_c6isr
    }
    ///0x1c4 - MDMA channel 6 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c6ifcr(&self) -> &MDMA_C6IFCR {
        &self.mdma_c6ifcr
    }
    ///0x1c8 - MDMA channel 6 error status register
    #[inline(always)]
    pub const fn mdma_c6esr(&self) -> &MDMA_C6ESR {
        &self.mdma_c6esr
    }
    ///0x1cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c6cr(&self) -> &MDMA_C6CR {
        &self.mdma_c6cr
    }
    /**0x1d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c6tcr(&self) -> &MDMA_C6TCR {
        &self.mdma_c6tcr
    }
    /**0x1d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c6bndtr(&self) -> &MDMA_C6BNDTR {
        &self.mdma_c6bndtr
    }
    /**0x1d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c6sar(&self) -> &MDMA_C6SAR {
        &self.mdma_c6sar
    }
    /**0x1dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c6dar(&self) -> &MDMA_C6DAR {
        &self.mdma_c6dar
    }
    /**0x1e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c6brur(&self) -> &MDMA_C6BRUR {
        &self.mdma_c6brur
    }
    /**0x1e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c6lar(&self) -> &MDMA_C6LAR {
        &self.mdma_c6lar
    }
    /**0x1e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c6tbr(&self) -> &MDMA_C6TBR {
        &self.mdma_c6tbr
    }
    /**0x1f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c6mar(&self) -> &MDMA_C6MAR {
        &self.mdma_c6mar
    }
    /**0x1f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c6mdr(&self) -> &MDMA_C6MDR {
        &self.mdma_c6mdr
    }
    ///0x200 - MDMA channel 7 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c7isr(&self) -> &MDMA_C7ISR {
        &self.mdma_c7isr
    }
    ///0x204 - MDMA channel 7 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c7ifcr(&self) -> &MDMA_C7IFCR {
        &self.mdma_c7ifcr
    }
    ///0x208 - MDMA channel 7 error status register
    #[inline(always)]
    pub const fn mdma_c7esr(&self) -> &MDMA_C7ESR {
        &self.mdma_c7esr
    }
    ///0x20c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c7cr(&self) -> &MDMA_C7CR {
        &self.mdma_c7cr
    }
    /**0x210 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c7tcr(&self) -> &MDMA_C7TCR {
        &self.mdma_c7tcr
    }
    /**0x214 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c7bndtr(&self) -> &MDMA_C7BNDTR {
        &self.mdma_c7bndtr
    }
    /**0x218 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c7sar(&self) -> &MDMA_C7SAR {
        &self.mdma_c7sar
    }
    /**0x21c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c7dar(&self) -> &MDMA_C7DAR {
        &self.mdma_c7dar
    }
    /**0x220 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c7brur(&self) -> &MDMA_C7BRUR {
        &self.mdma_c7brur
    }
    /**0x224 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c7lar(&self) -> &MDMA_C7LAR {
        &self.mdma_c7lar
    }
    /**0x228 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c7tbr(&self) -> &MDMA_C7TBR {
        &self.mdma_c7tbr
    }
    /**0x230 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c7mar(&self) -> &MDMA_C7MAR {
        &self.mdma_c7mar
    }
    /**0x234 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c7mdr(&self) -> &MDMA_C7MDR {
        &self.mdma_c7mdr
    }
    ///0x240 - MDMA channel 8 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c8isr(&self) -> &MDMA_C8ISR {
        &self.mdma_c8isr
    }
    ///0x244 - MDMA channel 8 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c8ifcr(&self) -> &MDMA_C8IFCR {
        &self.mdma_c8ifcr
    }
    ///0x248 - MDMA channel 8 error status register
    #[inline(always)]
    pub const fn mdma_c8esr(&self) -> &MDMA_C8ESR {
        &self.mdma_c8esr
    }
    ///0x24c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c8cr(&self) -> &MDMA_C8CR {
        &self.mdma_c8cr
    }
    /**0x250 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c8tcr(&self) -> &MDMA_C8TCR {
        &self.mdma_c8tcr
    }
    /**0x254 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c8bndtr(&self) -> &MDMA_C8BNDTR {
        &self.mdma_c8bndtr
    }
    /**0x258 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c8sar(&self) -> &MDMA_C8SAR {
        &self.mdma_c8sar
    }
    /**0x25c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c8dar(&self) -> &MDMA_C8DAR {
        &self.mdma_c8dar
    }
    /**0x260 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c8brur(&self) -> &MDMA_C8BRUR {
        &self.mdma_c8brur
    }
    /**0x264 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c8lar(&self) -> &MDMA_C8LAR {
        &self.mdma_c8lar
    }
    /**0x268 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c8tbr(&self) -> &MDMA_C8TBR {
        &self.mdma_c8tbr
    }
    /**0x270 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c8mar(&self) -> &MDMA_C8MAR {
        &self.mdma_c8mar
    }
    /**0x274 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c8mdr(&self) -> &MDMA_C8MDR {
        &self.mdma_c8mdr
    }
    ///0x280 - MDMA channel 9 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c9isr(&self) -> &MDMA_C9ISR {
        &self.mdma_c9isr
    }
    ///0x284 - MDMA channel 9 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c9ifcr(&self) -> &MDMA_C9IFCR {
        &self.mdma_c9ifcr
    }
    ///0x288 - MDMA channel 9 error status register
    #[inline(always)]
    pub const fn mdma_c9esr(&self) -> &MDMA_C9ESR {
        &self.mdma_c9esr
    }
    ///0x28c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c9cr(&self) -> &MDMA_C9CR {
        &self.mdma_c9cr
    }
    /**0x290 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c9tcr(&self) -> &MDMA_C9TCR {
        &self.mdma_c9tcr
    }
    /**0x294 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c9bndtr(&self) -> &MDMA_C9BNDTR {
        &self.mdma_c9bndtr
    }
    /**0x298 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c9sar(&self) -> &MDMA_C9SAR {
        &self.mdma_c9sar
    }
    /**0x29c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c9dar(&self) -> &MDMA_C9DAR {
        &self.mdma_c9dar
    }
    /**0x2a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c9brur(&self) -> &MDMA_C9BRUR {
        &self.mdma_c9brur
    }
    /**0x2a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c9lar(&self) -> &MDMA_C9LAR {
        &self.mdma_c9lar
    }
    /**0x2a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c9tbr(&self) -> &MDMA_C9TBR {
        &self.mdma_c9tbr
    }
    /**0x2b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c9mar(&self) -> &MDMA_C9MAR {
        &self.mdma_c9mar
    }
    /**0x2b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c9mdr(&self) -> &MDMA_C9MDR {
        &self.mdma_c9mdr
    }
    ///0x2c0 - MDMA channel 10 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c10isr(&self) -> &MDMA_C10ISR {
        &self.mdma_c10isr
    }
    ///0x2c4 - MDMA channel 10 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c10ifcr(&self) -> &MDMA_C10IFCR {
        &self.mdma_c10ifcr
    }
    ///0x2c8 - MDMA channel 10 error status register
    #[inline(always)]
    pub const fn mdma_c10esr(&self) -> &MDMA_C10ESR {
        &self.mdma_c10esr
    }
    ///0x2cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c10cr(&self) -> &MDMA_C10CR {
        &self.mdma_c10cr
    }
    /**0x2d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c10tcr(&self) -> &MDMA_C10TCR {
        &self.mdma_c10tcr
    }
    /**0x2d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c10bndtr(&self) -> &MDMA_C10BNDTR {
        &self.mdma_c10bndtr
    }
    /**0x2d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c10sar(&self) -> &MDMA_C10SAR {
        &self.mdma_c10sar
    }
    /**0x2dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c10dar(&self) -> &MDMA_C10DAR {
        &self.mdma_c10dar
    }
    /**0x2e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c10brur(&self) -> &MDMA_C10BRUR {
        &self.mdma_c10brur
    }
    /**0x2e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c10lar(&self) -> &MDMA_C10LAR {
        &self.mdma_c10lar
    }
    /**0x2e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c10tbr(&self) -> &MDMA_C10TBR {
        &self.mdma_c10tbr
    }
    /**0x2f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c10mar(&self) -> &MDMA_C10MAR {
        &self.mdma_c10mar
    }
    /**0x2f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c10mdr(&self) -> &MDMA_C10MDR {
        &self.mdma_c10mdr
    }
    ///0x300 - MDMA channel 11 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c11isr(&self) -> &MDMA_C11ISR {
        &self.mdma_c11isr
    }
    ///0x304 - MDMA channel 11 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c11ifcr(&self) -> &MDMA_C11IFCR {
        &self.mdma_c11ifcr
    }
    ///0x308 - MDMA channel 11 error status register
    #[inline(always)]
    pub const fn mdma_c11esr(&self) -> &MDMA_C11ESR {
        &self.mdma_c11esr
    }
    ///0x30c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c11cr(&self) -> &MDMA_C11CR {
        &self.mdma_c11cr
    }
    /**0x310 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c11tcr(&self) -> &MDMA_C11TCR {
        &self.mdma_c11tcr
    }
    /**0x314 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c11bndtr(&self) -> &MDMA_C11BNDTR {
        &self.mdma_c11bndtr
    }
    /**0x318 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c11sar(&self) -> &MDMA_C11SAR {
        &self.mdma_c11sar
    }
    /**0x31c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c11dar(&self) -> &MDMA_C11DAR {
        &self.mdma_c11dar
    }
    /**0x320 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c11brur(&self) -> &MDMA_C11BRUR {
        &self.mdma_c11brur
    }
    /**0x324 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c11lar(&self) -> &MDMA_C11LAR {
        &self.mdma_c11lar
    }
    /**0x328 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c11tbr(&self) -> &MDMA_C11TBR {
        &self.mdma_c11tbr
    }
    /**0x330 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c11mar(&self) -> &MDMA_C11MAR {
        &self.mdma_c11mar
    }
    /**0x334 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c11mdr(&self) -> &MDMA_C11MDR {
        &self.mdma_c11mdr
    }
    ///0x340 - MDMA channel 12 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c12isr(&self) -> &MDMA_C12ISR {
        &self.mdma_c12isr
    }
    ///0x344 - MDMA channel 12 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c12ifcr(&self) -> &MDMA_C12IFCR {
        &self.mdma_c12ifcr
    }
    ///0x348 - MDMA channel 12 error status register
    #[inline(always)]
    pub const fn mdma_c12esr(&self) -> &MDMA_C12ESR {
        &self.mdma_c12esr
    }
    ///0x34c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c12cr(&self) -> &MDMA_C12CR {
        &self.mdma_c12cr
    }
    /**0x350 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c12tcr(&self) -> &MDMA_C12TCR {
        &self.mdma_c12tcr
    }
    /**0x354 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c12bndtr(&self) -> &MDMA_C12BNDTR {
        &self.mdma_c12bndtr
    }
    /**0x358 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c12sar(&self) -> &MDMA_C12SAR {
        &self.mdma_c12sar
    }
    /**0x35c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c12dar(&self) -> &MDMA_C12DAR {
        &self.mdma_c12dar
    }
    /**0x360 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c12brur(&self) -> &MDMA_C12BRUR {
        &self.mdma_c12brur
    }
    /**0x364 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c12lar(&self) -> &MDMA_C12LAR {
        &self.mdma_c12lar
    }
    /**0x368 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c12tbr(&self) -> &MDMA_C12TBR {
        &self.mdma_c12tbr
    }
    /**0x370 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c12mar(&self) -> &MDMA_C12MAR {
        &self.mdma_c12mar
    }
    /**0x374 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c12mdr(&self) -> &MDMA_C12MDR {
        &self.mdma_c12mdr
    }
    ///0x380 - MDMA channel 13 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c13isr(&self) -> &MDMA_C13ISR {
        &self.mdma_c13isr
    }
    ///0x384 - MDMA channel 13 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c13ifcr(&self) -> &MDMA_C13IFCR {
        &self.mdma_c13ifcr
    }
    ///0x388 - MDMA channel 13 error status register
    #[inline(always)]
    pub const fn mdma_c13esr(&self) -> &MDMA_C13ESR {
        &self.mdma_c13esr
    }
    ///0x38c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c13cr(&self) -> &MDMA_C13CR {
        &self.mdma_c13cr
    }
    /**0x390 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c13tcr(&self) -> &MDMA_C13TCR {
        &self.mdma_c13tcr
    }
    /**0x394 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c13bndtr(&self) -> &MDMA_C13BNDTR {
        &self.mdma_c13bndtr
    }
    /**0x398 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c13sar(&self) -> &MDMA_C13SAR {
        &self.mdma_c13sar
    }
    /**0x39c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c13dar(&self) -> &MDMA_C13DAR {
        &self.mdma_c13dar
    }
    /**0x3a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c13brur(&self) -> &MDMA_C13BRUR {
        &self.mdma_c13brur
    }
    /**0x3a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c13lar(&self) -> &MDMA_C13LAR {
        &self.mdma_c13lar
    }
    /**0x3a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c13tbr(&self) -> &MDMA_C13TBR {
        &self.mdma_c13tbr
    }
    /**0x3b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c13mar(&self) -> &MDMA_C13MAR {
        &self.mdma_c13mar
    }
    /**0x3b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c13mdr(&self) -> &MDMA_C13MDR {
        &self.mdma_c13mdr
    }
    ///0x3c0 - MDMA channel 14 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c14isr(&self) -> &MDMA_C14ISR {
        &self.mdma_c14isr
    }
    ///0x3c4 - MDMA channel 14 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c14ifcr(&self) -> &MDMA_C14IFCR {
        &self.mdma_c14ifcr
    }
    ///0x3c8 - MDMA channel 14 error status register
    #[inline(always)]
    pub const fn mdma_c14esr(&self) -> &MDMA_C14ESR {
        &self.mdma_c14esr
    }
    ///0x3cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c14cr(&self) -> &MDMA_C14CR {
        &self.mdma_c14cr
    }
    /**0x3d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c14tcr(&self) -> &MDMA_C14TCR {
        &self.mdma_c14tcr
    }
    /**0x3d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c14bndtr(&self) -> &MDMA_C14BNDTR {
        &self.mdma_c14bndtr
    }
    /**0x3d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c14sar(&self) -> &MDMA_C14SAR {
        &self.mdma_c14sar
    }
    /**0x3dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c14dar(&self) -> &MDMA_C14DAR {
        &self.mdma_c14dar
    }
    /**0x3e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c14brur(&self) -> &MDMA_C14BRUR {
        &self.mdma_c14brur
    }
    /**0x3e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c14lar(&self) -> &MDMA_C14LAR {
        &self.mdma_c14lar
    }
    /**0x3e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c14tbr(&self) -> &MDMA_C14TBR {
        &self.mdma_c14tbr
    }
    /**0x3f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c14mar(&self) -> &MDMA_C14MAR {
        &self.mdma_c14mar
    }
    /**0x3f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c14mdr(&self) -> &MDMA_C14MDR {
        &self.mdma_c14mdr
    }
    ///0x400 - MDMA channel 15 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c15isr(&self) -> &MDMA_C15ISR {
        &self.mdma_c15isr
    }
    ///0x404 - MDMA channel 15 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c15ifcr(&self) -> &MDMA_C15IFCR {
        &self.mdma_c15ifcr
    }
    ///0x408 - MDMA channel 15 error status register
    #[inline(always)]
    pub const fn mdma_c15esr(&self) -> &MDMA_C15ESR {
        &self.mdma_c15esr
    }
    ///0x40c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c15cr(&self) -> &MDMA_C15CR {
        &self.mdma_c15cr
    }
    /**0x410 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c15tcr(&self) -> &MDMA_C15TCR {
        &self.mdma_c15tcr
    }
    /**0x414 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c15bndtr(&self) -> &MDMA_C15BNDTR {
        &self.mdma_c15bndtr
    }
    /**0x418 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c15sar(&self) -> &MDMA_C15SAR {
        &self.mdma_c15sar
    }
    /**0x41c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c15dar(&self) -> &MDMA_C15DAR {
        &self.mdma_c15dar
    }
    /**0x420 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c15brur(&self) -> &MDMA_C15BRUR {
        &self.mdma_c15brur
    }
    /**0x424 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c15lar(&self) -> &MDMA_C15LAR {
        &self.mdma_c15lar
    }
    /**0x428 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c15tbr(&self) -> &MDMA_C15TBR {
        &self.mdma_c15tbr
    }
    /**0x430 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c15mar(&self) -> &MDMA_C15MAR {
        &self.mdma_c15mar
    }
    /**0x434 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c15mdr(&self) -> &MDMA_C15MDR {
        &self.mdma_c15mdr
    }
    ///0x440 - MDMA channel 16 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c16isr(&self) -> &MDMA_C16ISR {
        &self.mdma_c16isr
    }
    ///0x444 - MDMA channel 16 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c16ifcr(&self) -> &MDMA_C16IFCR {
        &self.mdma_c16ifcr
    }
    ///0x448 - MDMA channel 16 error status register
    #[inline(always)]
    pub const fn mdma_c16esr(&self) -> &MDMA_C16ESR {
        &self.mdma_c16esr
    }
    ///0x44c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c16cr(&self) -> &MDMA_C16CR {
        &self.mdma_c16cr
    }
    /**0x450 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c16tcr(&self) -> &MDMA_C16TCR {
        &self.mdma_c16tcr
    }
    /**0x454 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c16bndtr(&self) -> &MDMA_C16BNDTR {
        &self.mdma_c16bndtr
    }
    /**0x458 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c16sar(&self) -> &MDMA_C16SAR {
        &self.mdma_c16sar
    }
    /**0x45c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c16dar(&self) -> &MDMA_C16DAR {
        &self.mdma_c16dar
    }
    /**0x460 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c16brur(&self) -> &MDMA_C16BRUR {
        &self.mdma_c16brur
    }
    /**0x464 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c16lar(&self) -> &MDMA_C16LAR {
        &self.mdma_c16lar
    }
    /**0x468 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c16tbr(&self) -> &MDMA_C16TBR {
        &self.mdma_c16tbr
    }
    /**0x470 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c16mar(&self) -> &MDMA_C16MAR {
        &self.mdma_c16mar
    }
    /**0x474 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c16mdr(&self) -> &MDMA_C16MDR {
        &self.mdma_c16mdr
    }
    ///0x480 - MDMA channel 17 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c17isr(&self) -> &MDMA_C17ISR {
        &self.mdma_c17isr
    }
    ///0x484 - MDMA channel 17 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c17ifcr(&self) -> &MDMA_C17IFCR {
        &self.mdma_c17ifcr
    }
    ///0x488 - MDMA channel 17 error status register
    #[inline(always)]
    pub const fn mdma_c17esr(&self) -> &MDMA_C17ESR {
        &self.mdma_c17esr
    }
    ///0x48c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c17cr(&self) -> &MDMA_C17CR {
        &self.mdma_c17cr
    }
    /**0x490 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c17tcr(&self) -> &MDMA_C17TCR {
        &self.mdma_c17tcr
    }
    /**0x494 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c17bndtr(&self) -> &MDMA_C17BNDTR {
        &self.mdma_c17bndtr
    }
    /**0x498 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c17sar(&self) -> &MDMA_C17SAR {
        &self.mdma_c17sar
    }
    /**0x49c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c17dar(&self) -> &MDMA_C17DAR {
        &self.mdma_c17dar
    }
    /**0x4a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c17brur(&self) -> &MDMA_C17BRUR {
        &self.mdma_c17brur
    }
    /**0x4a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c17lar(&self) -> &MDMA_C17LAR {
        &self.mdma_c17lar
    }
    /**0x4a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c17tbr(&self) -> &MDMA_C17TBR {
        &self.mdma_c17tbr
    }
    /**0x4b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c17mar(&self) -> &MDMA_C17MAR {
        &self.mdma_c17mar
    }
    /**0x4b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c17mdr(&self) -> &MDMA_C17MDR {
        &self.mdma_c17mdr
    }
    ///0x4c0 - MDMA channel 18 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c18isr(&self) -> &MDMA_C18ISR {
        &self.mdma_c18isr
    }
    ///0x4c4 - MDMA channel 18 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c18ifcr(&self) -> &MDMA_C18IFCR {
        &self.mdma_c18ifcr
    }
    ///0x4c8 - MDMA channel 18 error status register
    #[inline(always)]
    pub const fn mdma_c18esr(&self) -> &MDMA_C18ESR {
        &self.mdma_c18esr
    }
    ///0x4cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c18cr(&self) -> &MDMA_C18CR {
        &self.mdma_c18cr
    }
    /**0x4d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c18tcr(&self) -> &MDMA_C18TCR {
        &self.mdma_c18tcr
    }
    /**0x4d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c18bndtr(&self) -> &MDMA_C18BNDTR {
        &self.mdma_c18bndtr
    }
    /**0x4d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c18sar(&self) -> &MDMA_C18SAR {
        &self.mdma_c18sar
    }
    /**0x4dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c18dar(&self) -> &MDMA_C18DAR {
        &self.mdma_c18dar
    }
    /**0x4e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c18brur(&self) -> &MDMA_C18BRUR {
        &self.mdma_c18brur
    }
    /**0x4e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c18lar(&self) -> &MDMA_C18LAR {
        &self.mdma_c18lar
    }
    /**0x4e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c18tbr(&self) -> &MDMA_C18TBR {
        &self.mdma_c18tbr
    }
    /**0x4f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c18mar(&self) -> &MDMA_C18MAR {
        &self.mdma_c18mar
    }
    /**0x4f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c18mdr(&self) -> &MDMA_C18MDR {
        &self.mdma_c18mdr
    }
    ///0x500 - MDMA channel 19 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c19isr(&self) -> &MDMA_C19ISR {
        &self.mdma_c19isr
    }
    ///0x504 - MDMA channel 19 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c19ifcr(&self) -> &MDMA_C19IFCR {
        &self.mdma_c19ifcr
    }
    ///0x508 - MDMA channel 19 error status register
    #[inline(always)]
    pub const fn mdma_c19esr(&self) -> &MDMA_C19ESR {
        &self.mdma_c19esr
    }
    ///0x50c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c19cr(&self) -> &MDMA_C19CR {
        &self.mdma_c19cr
    }
    /**0x510 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c19tcr(&self) -> &MDMA_C19TCR {
        &self.mdma_c19tcr
    }
    /**0x514 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c19bndtr(&self) -> &MDMA_C19BNDTR {
        &self.mdma_c19bndtr
    }
    /**0x518 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c19sar(&self) -> &MDMA_C19SAR {
        &self.mdma_c19sar
    }
    /**0x51c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c19dar(&self) -> &MDMA_C19DAR {
        &self.mdma_c19dar
    }
    /**0x520 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c19brur(&self) -> &MDMA_C19BRUR {
        &self.mdma_c19brur
    }
    /**0x524 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c19lar(&self) -> &MDMA_C19LAR {
        &self.mdma_c19lar
    }
    /**0x528 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c19tbr(&self) -> &MDMA_C19TBR {
        &self.mdma_c19tbr
    }
    /**0x530 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c19mar(&self) -> &MDMA_C19MAR {
        &self.mdma_c19mar
    }
    /**0x534 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c19mdr(&self) -> &MDMA_C19MDR {
        &self.mdma_c19mdr
    }
    ///0x540 - MDMA channel 20 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c20isr(&self) -> &MDMA_C20ISR {
        &self.mdma_c20isr
    }
    ///0x544 - MDMA channel 20 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c20ifcr(&self) -> &MDMA_C20IFCR {
        &self.mdma_c20ifcr
    }
    ///0x548 - MDMA channel 20 error status register
    #[inline(always)]
    pub const fn mdma_c20esr(&self) -> &MDMA_C20ESR {
        &self.mdma_c20esr
    }
    ///0x54c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c20cr(&self) -> &MDMA_C20CR {
        &self.mdma_c20cr
    }
    /**0x550 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c20tcr(&self) -> &MDMA_C20TCR {
        &self.mdma_c20tcr
    }
    /**0x554 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c20bndtr(&self) -> &MDMA_C20BNDTR {
        &self.mdma_c20bndtr
    }
    /**0x558 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c20sar(&self) -> &MDMA_C20SAR {
        &self.mdma_c20sar
    }
    /**0x55c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c20dar(&self) -> &MDMA_C20DAR {
        &self.mdma_c20dar
    }
    /**0x560 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c20brur(&self) -> &MDMA_C20BRUR {
        &self.mdma_c20brur
    }
    /**0x564 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c20lar(&self) -> &MDMA_C20LAR {
        &self.mdma_c20lar
    }
    /**0x568 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c20tbr(&self) -> &MDMA_C20TBR {
        &self.mdma_c20tbr
    }
    /**0x570 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c20mar(&self) -> &MDMA_C20MAR {
        &self.mdma_c20mar
    }
    /**0x574 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c20mdr(&self) -> &MDMA_C20MDR {
        &self.mdma_c20mdr
    }
    ///0x580 - MDMA channel 21 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c21isr(&self) -> &MDMA_C21ISR {
        &self.mdma_c21isr
    }
    ///0x584 - MDMA channel 21 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c21ifcr(&self) -> &MDMA_C21IFCR {
        &self.mdma_c21ifcr
    }
    ///0x588 - MDMA channel 21 error status register
    #[inline(always)]
    pub const fn mdma_c21esr(&self) -> &MDMA_C21ESR {
        &self.mdma_c21esr
    }
    ///0x58c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c21cr(&self) -> &MDMA_C21CR {
        &self.mdma_c21cr
    }
    /**0x590 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c21tcr(&self) -> &MDMA_C21TCR {
        &self.mdma_c21tcr
    }
    /**0x594 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c21bndtr(&self) -> &MDMA_C21BNDTR {
        &self.mdma_c21bndtr
    }
    /**0x598 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c21sar(&self) -> &MDMA_C21SAR {
        &self.mdma_c21sar
    }
    /**0x59c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c21dar(&self) -> &MDMA_C21DAR {
        &self.mdma_c21dar
    }
    /**0x5a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c21brur(&self) -> &MDMA_C21BRUR {
        &self.mdma_c21brur
    }
    /**0x5a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c21lar(&self) -> &MDMA_C21LAR {
        &self.mdma_c21lar
    }
    /**0x5a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c21tbr(&self) -> &MDMA_C21TBR {
        &self.mdma_c21tbr
    }
    /**0x5b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c21mar(&self) -> &MDMA_C21MAR {
        &self.mdma_c21mar
    }
    /**0x5b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c21mdr(&self) -> &MDMA_C21MDR {
        &self.mdma_c21mdr
    }
    ///0x5c0 - MDMA channel 22 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c22isr(&self) -> &MDMA_C22ISR {
        &self.mdma_c22isr
    }
    ///0x5c4 - MDMA channel 22 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c22ifcr(&self) -> &MDMA_C22IFCR {
        &self.mdma_c22ifcr
    }
    ///0x5c8 - MDMA channel 22 error status register
    #[inline(always)]
    pub const fn mdma_c22esr(&self) -> &MDMA_C22ESR {
        &self.mdma_c22esr
    }
    ///0x5cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c22cr(&self) -> &MDMA_C22CR {
        &self.mdma_c22cr
    }
    /**0x5d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c22tcr(&self) -> &MDMA_C22TCR {
        &self.mdma_c22tcr
    }
    /**0x5d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c22bndtr(&self) -> &MDMA_C22BNDTR {
        &self.mdma_c22bndtr
    }
    /**0x5d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c22sar(&self) -> &MDMA_C22SAR {
        &self.mdma_c22sar
    }
    /**0x5dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c22dar(&self) -> &MDMA_C22DAR {
        &self.mdma_c22dar
    }
    /**0x5e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c22brur(&self) -> &MDMA_C22BRUR {
        &self.mdma_c22brur
    }
    /**0x5e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c22lar(&self) -> &MDMA_C22LAR {
        &self.mdma_c22lar
    }
    /**0x5e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c22tbr(&self) -> &MDMA_C22TBR {
        &self.mdma_c22tbr
    }
    /**0x5f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c22mar(&self) -> &MDMA_C22MAR {
        &self.mdma_c22mar
    }
    /**0x5f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c22mdr(&self) -> &MDMA_C22MDR {
        &self.mdma_c22mdr
    }
    ///0x600 - MDMA channel 23 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c23isr(&self) -> &MDMA_C23ISR {
        &self.mdma_c23isr
    }
    ///0x604 - MDMA channel 23 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c23ifcr(&self) -> &MDMA_C23IFCR {
        &self.mdma_c23ifcr
    }
    ///0x608 - MDMA channel 23 error status register
    #[inline(always)]
    pub const fn mdma_c23esr(&self) -> &MDMA_C23ESR {
        &self.mdma_c23esr
    }
    ///0x60c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c23cr(&self) -> &MDMA_C23CR {
        &self.mdma_c23cr
    }
    /**0x610 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c23tcr(&self) -> &MDMA_C23TCR {
        &self.mdma_c23tcr
    }
    /**0x614 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c23bndtr(&self) -> &MDMA_C23BNDTR {
        &self.mdma_c23bndtr
    }
    /**0x618 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c23sar(&self) -> &MDMA_C23SAR {
        &self.mdma_c23sar
    }
    /**0x61c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c23dar(&self) -> &MDMA_C23DAR {
        &self.mdma_c23dar
    }
    /**0x620 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c23brur(&self) -> &MDMA_C23BRUR {
        &self.mdma_c23brur
    }
    /**0x624 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c23lar(&self) -> &MDMA_C23LAR {
        &self.mdma_c23lar
    }
    /**0x628 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c23tbr(&self) -> &MDMA_C23TBR {
        &self.mdma_c23tbr
    }
    /**0x630 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c23mar(&self) -> &MDMA_C23MAR {
        &self.mdma_c23mar
    }
    /**0x634 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c23mdr(&self) -> &MDMA_C23MDR {
        &self.mdma_c23mdr
    }
    ///0x640 - MDMA channel 24 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c24isr(&self) -> &MDMA_C24ISR {
        &self.mdma_c24isr
    }
    ///0x644 - MDMA channel 24 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c24ifcr(&self) -> &MDMA_C24IFCR {
        &self.mdma_c24ifcr
    }
    ///0x648 - MDMA channel 24 error status register
    #[inline(always)]
    pub const fn mdma_c24esr(&self) -> &MDMA_C24ESR {
        &self.mdma_c24esr
    }
    ///0x64c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c24cr(&self) -> &MDMA_C24CR {
        &self.mdma_c24cr
    }
    /**0x650 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c24tcr(&self) -> &MDMA_C24TCR {
        &self.mdma_c24tcr
    }
    /**0x654 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c24bndtr(&self) -> &MDMA_C24BNDTR {
        &self.mdma_c24bndtr
    }
    /**0x658 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c24sar(&self) -> &MDMA_C24SAR {
        &self.mdma_c24sar
    }
    /**0x65c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c24dar(&self) -> &MDMA_C24DAR {
        &self.mdma_c24dar
    }
    /**0x660 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c24brur(&self) -> &MDMA_C24BRUR {
        &self.mdma_c24brur
    }
    /**0x664 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c24lar(&self) -> &MDMA_C24LAR {
        &self.mdma_c24lar
    }
    /**0x668 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c24tbr(&self) -> &MDMA_C24TBR {
        &self.mdma_c24tbr
    }
    /**0x670 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c24mar(&self) -> &MDMA_C24MAR {
        &self.mdma_c24mar
    }
    /**0x674 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c24mdr(&self) -> &MDMA_C24MDR {
        &self.mdma_c24mdr
    }
    ///0x680 - MDMA channel 25 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c25isr(&self) -> &MDMA_C25ISR {
        &self.mdma_c25isr
    }
    ///0x684 - MDMA channel 25 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c25ifcr(&self) -> &MDMA_C25IFCR {
        &self.mdma_c25ifcr
    }
    ///0x688 - MDMA channel 25 error status register
    #[inline(always)]
    pub const fn mdma_c25esr(&self) -> &MDMA_C25ESR {
        &self.mdma_c25esr
    }
    ///0x68c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c25cr(&self) -> &MDMA_C25CR {
        &self.mdma_c25cr
    }
    /**0x690 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c25tcr(&self) -> &MDMA_C25TCR {
        &self.mdma_c25tcr
    }
    /**0x694 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c25bndtr(&self) -> &MDMA_C25BNDTR {
        &self.mdma_c25bndtr
    }
    /**0x698 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c25sar(&self) -> &MDMA_C25SAR {
        &self.mdma_c25sar
    }
    /**0x69c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c25dar(&self) -> &MDMA_C25DAR {
        &self.mdma_c25dar
    }
    /**0x6a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c25brur(&self) -> &MDMA_C25BRUR {
        &self.mdma_c25brur
    }
    /**0x6a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c25lar(&self) -> &MDMA_C25LAR {
        &self.mdma_c25lar
    }
    /**0x6a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c25tbr(&self) -> &MDMA_C25TBR {
        &self.mdma_c25tbr
    }
    /**0x6b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c25mar(&self) -> &MDMA_C25MAR {
        &self.mdma_c25mar
    }
    /**0x6b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c25mdr(&self) -> &MDMA_C25MDR {
        &self.mdma_c25mdr
    }
    ///0x6c0 - MDMA channel 26 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c26isr(&self) -> &MDMA_C26ISR {
        &self.mdma_c26isr
    }
    ///0x6c4 - MDMA channel 26 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c26ifcr(&self) -> &MDMA_C26IFCR {
        &self.mdma_c26ifcr
    }
    ///0x6c8 - MDMA channel 26 error status register
    #[inline(always)]
    pub const fn mdma_c26esr(&self) -> &MDMA_C26ESR {
        &self.mdma_c26esr
    }
    ///0x6cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c26cr(&self) -> &MDMA_C26CR {
        &self.mdma_c26cr
    }
    /**0x6d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c26tcr(&self) -> &MDMA_C26TCR {
        &self.mdma_c26tcr
    }
    /**0x6d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c26bndtr(&self) -> &MDMA_C26BNDTR {
        &self.mdma_c26bndtr
    }
    /**0x6d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c26sar(&self) -> &MDMA_C26SAR {
        &self.mdma_c26sar
    }
    /**0x6dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c26dar(&self) -> &MDMA_C26DAR {
        &self.mdma_c26dar
    }
    /**0x6e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c26brur(&self) -> &MDMA_C26BRUR {
        &self.mdma_c26brur
    }
    /**0x6e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c26lar(&self) -> &MDMA_C26LAR {
        &self.mdma_c26lar
    }
    /**0x6e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c26tbr(&self) -> &MDMA_C26TBR {
        &self.mdma_c26tbr
    }
    /**0x6f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c26mar(&self) -> &MDMA_C26MAR {
        &self.mdma_c26mar
    }
    /**0x6f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c26mdr(&self) -> &MDMA_C26MDR {
        &self.mdma_c26mdr
    }
    ///0x700 - MDMA channel 27 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c27isr(&self) -> &MDMA_C27ISR {
        &self.mdma_c27isr
    }
    ///0x704 - MDMA channel 27 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c27ifcr(&self) -> &MDMA_C27IFCR {
        &self.mdma_c27ifcr
    }
    ///0x708 - MDMA channel 27 error status register
    #[inline(always)]
    pub const fn mdma_c27esr(&self) -> &MDMA_C27ESR {
        &self.mdma_c27esr
    }
    ///0x70c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c27cr(&self) -> &MDMA_C27CR {
        &self.mdma_c27cr
    }
    /**0x710 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c27tcr(&self) -> &MDMA_C27TCR {
        &self.mdma_c27tcr
    }
    /**0x714 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c27bndtr(&self) -> &MDMA_C27BNDTR {
        &self.mdma_c27bndtr
    }
    /**0x718 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c27sar(&self) -> &MDMA_C27SAR {
        &self.mdma_c27sar
    }
    /**0x71c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c27dar(&self) -> &MDMA_C27DAR {
        &self.mdma_c27dar
    }
    /**0x720 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c27brur(&self) -> &MDMA_C27BRUR {
        &self.mdma_c27brur
    }
    /**0x724 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c27lar(&self) -> &MDMA_C27LAR {
        &self.mdma_c27lar
    }
    /**0x728 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c27tbr(&self) -> &MDMA_C27TBR {
        &self.mdma_c27tbr
    }
    /**0x730 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c27mar(&self) -> &MDMA_C27MAR {
        &self.mdma_c27mar
    }
    /**0x734 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c27mdr(&self) -> &MDMA_C27MDR {
        &self.mdma_c27mdr
    }
    ///0x740 - MDMA channel 28 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c28isr(&self) -> &MDMA_C28ISR {
        &self.mdma_c28isr
    }
    ///0x744 - MDMA channel 28 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c28ifcr(&self) -> &MDMA_C28IFCR {
        &self.mdma_c28ifcr
    }
    ///0x748 - MDMA channel 28 error status register
    #[inline(always)]
    pub const fn mdma_c28esr(&self) -> &MDMA_C28ESR {
        &self.mdma_c28esr
    }
    ///0x74c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c28cr(&self) -> &MDMA_C28CR {
        &self.mdma_c28cr
    }
    /**0x750 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c28tcr(&self) -> &MDMA_C28TCR {
        &self.mdma_c28tcr
    }
    /**0x754 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c28bndtr(&self) -> &MDMA_C28BNDTR {
        &self.mdma_c28bndtr
    }
    /**0x758 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c28sar(&self) -> &MDMA_C28SAR {
        &self.mdma_c28sar
    }
    /**0x75c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c28dar(&self) -> &MDMA_C28DAR {
        &self.mdma_c28dar
    }
    /**0x760 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c28brur(&self) -> &MDMA_C28BRUR {
        &self.mdma_c28brur
    }
    /**0x764 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c28lar(&self) -> &MDMA_C28LAR {
        &self.mdma_c28lar
    }
    /**0x768 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c28tbr(&self) -> &MDMA_C28TBR {
        &self.mdma_c28tbr
    }
    /**0x770 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c28mar(&self) -> &MDMA_C28MAR {
        &self.mdma_c28mar
    }
    /**0x774 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c28mdr(&self) -> &MDMA_C28MDR {
        &self.mdma_c28mdr
    }
    ///0x780 - MDMA channel 29 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c29isr(&self) -> &MDMA_C29ISR {
        &self.mdma_c29isr
    }
    ///0x784 - MDMA channel 29 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c29ifcr(&self) -> &MDMA_C29IFCR {
        &self.mdma_c29ifcr
    }
    ///0x788 - MDMA channel 29 error status register
    #[inline(always)]
    pub const fn mdma_c29esr(&self) -> &MDMA_C29ESR {
        &self.mdma_c29esr
    }
    ///0x78c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c29cr(&self) -> &MDMA_C29CR {
        &self.mdma_c29cr
    }
    /**0x790 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c29tcr(&self) -> &MDMA_C29TCR {
        &self.mdma_c29tcr
    }
    /**0x794 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c29bndtr(&self) -> &MDMA_C29BNDTR {
        &self.mdma_c29bndtr
    }
    /**0x798 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c29sar(&self) -> &MDMA_C29SAR {
        &self.mdma_c29sar
    }
    /**0x79c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c29dar(&self) -> &MDMA_C29DAR {
        &self.mdma_c29dar
    }
    /**0x7a0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c29brur(&self) -> &MDMA_C29BRUR {
        &self.mdma_c29brur
    }
    /**0x7a4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c29lar(&self) -> &MDMA_C29LAR {
        &self.mdma_c29lar
    }
    /**0x7a8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c29tbr(&self) -> &MDMA_C29TBR {
        &self.mdma_c29tbr
    }
    /**0x7b0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c29mar(&self) -> &MDMA_C29MAR {
        &self.mdma_c29mar
    }
    /**0x7b4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c29mdr(&self) -> &MDMA_C29MDR {
        &self.mdma_c29mdr
    }
    ///0x7c0 - MDMA channel 30 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c30isr(&self) -> &MDMA_C30ISR {
        &self.mdma_c30isr
    }
    ///0x7c4 - MDMA channel 30 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c30ifcr(&self) -> &MDMA_C30IFCR {
        &self.mdma_c30ifcr
    }
    ///0x7c8 - MDMA channel 30 error status register
    #[inline(always)]
    pub const fn mdma_c30esr(&self) -> &MDMA_C30ESR {
        &self.mdma_c30esr
    }
    ///0x7cc - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c30cr(&self) -> &MDMA_C30CR {
        &self.mdma_c30cr
    }
    /**0x7d0 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c30tcr(&self) -> &MDMA_C30TCR {
        &self.mdma_c30tcr
    }
    /**0x7d4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c30bndtr(&self) -> &MDMA_C30BNDTR {
        &self.mdma_c30bndtr
    }
    /**0x7d8 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c30sar(&self) -> &MDMA_C30SAR {
        &self.mdma_c30sar
    }
    /**0x7dc - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c30dar(&self) -> &MDMA_C30DAR {
        &self.mdma_c30dar
    }
    /**0x7e0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c30brur(&self) -> &MDMA_C30BRUR {
        &self.mdma_c30brur
    }
    /**0x7e4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c30lar(&self) -> &MDMA_C30LAR {
        &self.mdma_c30lar
    }
    /**0x7e8 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c30tbr(&self) -> &MDMA_C30TBR {
        &self.mdma_c30tbr
    }
    /**0x7f0 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c30mar(&self) -> &MDMA_C30MAR {
        &self.mdma_c30mar
    }
    /**0x7f4 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c30mdr(&self) -> &MDMA_C30MDR {
        &self.mdma_c30mdr
    }
    ///0x800 - MDMA channel 31 interrupt/status register
    #[inline(always)]
    pub const fn mdma_c31isr(&self) -> &MDMA_C31ISR {
        &self.mdma_c31isr
    }
    ///0x804 - MDMA channel 31 interrupt flag clear register
    #[inline(always)]
    pub const fn mdma_c31ifcr(&self) -> &MDMA_C31IFCR {
        &self.mdma_c31ifcr
    }
    ///0x808 - MDMA channel 31 error status register
    #[inline(always)]
    pub const fn mdma_c31esr(&self) -> &MDMA_C31ESR {
        &self.mdma_c31esr
    }
    ///0x80c - This register is used to control the concerned channel.
    #[inline(always)]
    pub const fn mdma_c31cr(&self) -> &MDMA_C31CR {
        &self.mdma_c31cr
    }
    /**0x810 - This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x00).*/
    #[inline(always)]
    pub const fn mdma_c31tcr(&self) -> &MDMA_C31TCR {
        &self.mdma_c31tcr
    }
    /**0x814 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x04).*/
    #[inline(always)]
    pub const fn mdma_c31bndtr(&self) -> &MDMA_C31BNDTR {
        &self.mdma_c31bndtr
    }
    /**0x818 - In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x08).*/
    #[inline(always)]
    pub const fn mdma_c31sar(&self) -> &MDMA_C31SAR {
        &self.mdma_c31sar
    }
    /**0x81c - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x0C). M*/
    #[inline(always)]
    pub const fn mdma_c31dar(&self) -> &MDMA_C31DAR {
        &self.mdma_c31dar
    }
    /**0x820 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x10).*/
    #[inline(always)]
    pub const fn mdma_c31brur(&self) -> &MDMA_C31BRUR {
        &self.mdma_c31brur
    }
    /**0x824 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
    #[inline(always)]
    pub const fn mdma_c31lar(&self) -> &MDMA_C31LAR {
        &self.mdma_c31lar
    }
    /**0x828 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x18).*/
    #[inline(always)]
    pub const fn mdma_c31tbr(&self) -> &MDMA_C31TBR {
        &self.mdma_c31tbr
    }
    /**0x830 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x20).*/
    #[inline(always)]
    pub const fn mdma_c31mar(&self) -> &MDMA_C31MAR {
        &self.mdma_c31mar
    }
    /**0x834 - In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
    + 0x24).*/
    #[inline(always)]
    pub const fn mdma_c31mdr(&self) -> &MDMA_C31MDR {
        &self.mdma_c31mdr
    }
}
/**MDMA_GISR0 (r) register accessor: MDMA global interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_gisr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_GISR0)

For information about available fields see [`mod@mdma_gisr0`]
module*/
pub type MDMA_GISR0 = crate::Reg<mdma_gisr0::MDMA_GISR0rs>;
///MDMA global interrupt/status register
pub mod mdma_gisr0;
/**MDMA_SGISR0 (r) register accessor: MDMA secure global interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_sgisr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_SGISR0)

For information about available fields see [`mod@mdma_sgisr0`]
module*/
pub type MDMA_SGISR0 = crate::Reg<mdma_sgisr0::MDMA_SGISR0rs>;
///MDMA secure global interrupt/status register
pub mod mdma_sgisr0;
/**MDMA_C0ISR (r) register accessor: MDMA channel 0 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c0isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0ISR)

For information about available fields see [`mod@mdma_c0isr`]
module*/
pub type MDMA_C0ISR = crate::Reg<mdma_c0isr::MDMA_C0ISRrs>;
///MDMA channel 0 interrupt/status register
pub mod mdma_c0isr;
/**MDMA_C0IFCR (w) register accessor: MDMA channel 0 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0IFCR)

For information about available fields see [`mod@mdma_c0ifcr`]
module*/
pub type MDMA_C0IFCR = crate::Reg<mdma_c0ifcr::MDMA_C0IFCRrs>;
///MDMA channel 0 interrupt flag clear register
pub mod mdma_c0ifcr;
/**MDMA_C0ESR (r) register accessor: MDMA channel 0 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c0esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0ESR)

For information about available fields see [`mod@mdma_c0esr`]
module*/
pub type MDMA_C0ESR = crate::Reg<mdma_c0esr::MDMA_C0ESRrs>;
///MDMA channel 0 error status register
pub mod mdma_c0esr;
/**MDMA_C0CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0CR)

For information about available fields see [`mod@mdma_c0cr`]
module*/
pub type MDMA_C0CR = crate::Reg<mdma_c0cr::MDMA_C0CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c0cr;
/**MDMA_C0TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c0tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0TCR)

For information about available fields see [`mod@mdma_c0tcr`]
module*/
pub type MDMA_C0TCR = crate::Reg<mdma_c0tcr::MDMA_C0TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c0tcr;
/**MDMA_C0BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c0bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0BNDTR)

For information about available fields see [`mod@mdma_c0bndtr`]
module*/
pub type MDMA_C0BNDTR = crate::Reg<mdma_c0bndtr::MDMA_C0BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c0bndtr;
/**MDMA_C0SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c0sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0SAR)

For information about available fields see [`mod@mdma_c0sar`]
module*/
pub type MDMA_C0SAR = crate::Reg<mdma_c0sar::MDMA_C0SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c0sar;
/**MDMA_C0DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c0dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0DAR)

For information about available fields see [`mod@mdma_c0dar`]
module*/
pub type MDMA_C0DAR = crate::Reg<mdma_c0dar::MDMA_C0DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c0dar;
/**MDMA_C0BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c0brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0BRUR)

For information about available fields see [`mod@mdma_c0brur`]
module*/
pub type MDMA_C0BRUR = crate::Reg<mdma_c0brur::MDMA_C0BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c0brur;
/**MDMA_C0LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c0lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0LAR)

For information about available fields see [`mod@mdma_c0lar`]
module*/
pub type MDMA_C0LAR = crate::Reg<mdma_c0lar::MDMA_C0LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c0lar;
/**MDMA_C0TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c0tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0TBR)

For information about available fields see [`mod@mdma_c0tbr`]
module*/
pub type MDMA_C0TBR = crate::Reg<mdma_c0tbr::MDMA_C0TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c0tbr;
/**MDMA_C0MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c0mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0MAR)

For information about available fields see [`mod@mdma_c0mar`]
module*/
pub type MDMA_C0MAR = crate::Reg<mdma_c0mar::MDMA_C0MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c0mar;
/**MDMA_C0MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c0mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c0mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C0MDR)

For information about available fields see [`mod@mdma_c0mdr`]
module*/
pub type MDMA_C0MDR = crate::Reg<mdma_c0mdr::MDMA_C0MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c0mdr;
/**MDMA_C1ISR (r) register accessor: MDMA channel 1 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1ISR)

For information about available fields see [`mod@mdma_c1isr`]
module*/
pub type MDMA_C1ISR = crate::Reg<mdma_c1isr::MDMA_C1ISRrs>;
///MDMA channel 1 interrupt/status register
pub mod mdma_c1isr;
/**MDMA_C1IFCR (w) register accessor: MDMA channel 1 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1IFCR)

For information about available fields see [`mod@mdma_c1ifcr`]
module*/
pub type MDMA_C1IFCR = crate::Reg<mdma_c1ifcr::MDMA_C1IFCRrs>;
///MDMA channel 1 interrupt flag clear register
pub mod mdma_c1ifcr;
/**MDMA_C1ESR (r) register accessor: MDMA channel 1 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c1esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1ESR)

For information about available fields see [`mod@mdma_c1esr`]
module*/
pub type MDMA_C1ESR = crate::Reg<mdma_c1esr::MDMA_C1ESRrs>;
///MDMA channel 1 error status register
pub mod mdma_c1esr;
/**MDMA_C1CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1CR)

For information about available fields see [`mod@mdma_c1cr`]
module*/
pub type MDMA_C1CR = crate::Reg<mdma_c1cr::MDMA_C1CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c1cr;
/**MDMA_C1TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c1tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1TCR)

For information about available fields see [`mod@mdma_c1tcr`]
module*/
pub type MDMA_C1TCR = crate::Reg<mdma_c1tcr::MDMA_C1TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c1tcr;
/**MDMA_C1BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c1bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1BNDTR)

For information about available fields see [`mod@mdma_c1bndtr`]
module*/
pub type MDMA_C1BNDTR = crate::Reg<mdma_c1bndtr::MDMA_C1BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c1bndtr;
/**MDMA_C1SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c1sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1SAR)

For information about available fields see [`mod@mdma_c1sar`]
module*/
pub type MDMA_C1SAR = crate::Reg<mdma_c1sar::MDMA_C1SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c1sar;
/**MDMA_C1DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c1dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1DAR)

For information about available fields see [`mod@mdma_c1dar`]
module*/
pub type MDMA_C1DAR = crate::Reg<mdma_c1dar::MDMA_C1DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c1dar;
/**MDMA_C1BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c1brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1BRUR)

For information about available fields see [`mod@mdma_c1brur`]
module*/
pub type MDMA_C1BRUR = crate::Reg<mdma_c1brur::MDMA_C1BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c1brur;
/**MDMA_C1LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c1lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1LAR)

For information about available fields see [`mod@mdma_c1lar`]
module*/
pub type MDMA_C1LAR = crate::Reg<mdma_c1lar::MDMA_C1LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c1lar;
/**MDMA_C1TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c1tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1TBR)

For information about available fields see [`mod@mdma_c1tbr`]
module*/
pub type MDMA_C1TBR = crate::Reg<mdma_c1tbr::MDMA_C1TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c1tbr;
/**MDMA_C1MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c1mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1MAR)

For information about available fields see [`mod@mdma_c1mar`]
module*/
pub type MDMA_C1MAR = crate::Reg<mdma_c1mar::MDMA_C1MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c1mar;
/**MDMA_C1MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c1mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C1MDR)

For information about available fields see [`mod@mdma_c1mdr`]
module*/
pub type MDMA_C1MDR = crate::Reg<mdma_c1mdr::MDMA_C1MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c1mdr;
/**MDMA_C2ISR (r) register accessor: MDMA channel 2 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2ISR)

For information about available fields see [`mod@mdma_c2isr`]
module*/
pub type MDMA_C2ISR = crate::Reg<mdma_c2isr::MDMA_C2ISRrs>;
///MDMA channel 2 interrupt/status register
pub mod mdma_c2isr;
/**MDMA_C2IFCR (w) register accessor: MDMA channel 2 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2IFCR)

For information about available fields see [`mod@mdma_c2ifcr`]
module*/
pub type MDMA_C2IFCR = crate::Reg<mdma_c2ifcr::MDMA_C2IFCRrs>;
///MDMA channel 2 interrupt flag clear register
pub mod mdma_c2ifcr;
/**MDMA_C2ESR (r) register accessor: MDMA channel 2 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c2esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2ESR)

For information about available fields see [`mod@mdma_c2esr`]
module*/
pub type MDMA_C2ESR = crate::Reg<mdma_c2esr::MDMA_C2ESRrs>;
///MDMA channel 2 error status register
pub mod mdma_c2esr;
/**MDMA_C2CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2CR)

For information about available fields see [`mod@mdma_c2cr`]
module*/
pub type MDMA_C2CR = crate::Reg<mdma_c2cr::MDMA_C2CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c2cr;
/**MDMA_C2TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c2tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2TCR)

For information about available fields see [`mod@mdma_c2tcr`]
module*/
pub type MDMA_C2TCR = crate::Reg<mdma_c2tcr::MDMA_C2TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c2tcr;
/**MDMA_C2BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c2bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2BNDTR)

For information about available fields see [`mod@mdma_c2bndtr`]
module*/
pub type MDMA_C2BNDTR = crate::Reg<mdma_c2bndtr::MDMA_C2BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c2bndtr;
/**MDMA_C2SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c2sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2SAR)

For information about available fields see [`mod@mdma_c2sar`]
module*/
pub type MDMA_C2SAR = crate::Reg<mdma_c2sar::MDMA_C2SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c2sar;
/**MDMA_C2DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c2dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2DAR)

For information about available fields see [`mod@mdma_c2dar`]
module*/
pub type MDMA_C2DAR = crate::Reg<mdma_c2dar::MDMA_C2DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c2dar;
/**MDMA_C2BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c2brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2BRUR)

For information about available fields see [`mod@mdma_c2brur`]
module*/
pub type MDMA_C2BRUR = crate::Reg<mdma_c2brur::MDMA_C2BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c2brur;
/**MDMA_C2LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c2lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2LAR)

For information about available fields see [`mod@mdma_c2lar`]
module*/
pub type MDMA_C2LAR = crate::Reg<mdma_c2lar::MDMA_C2LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c2lar;
/**MDMA_C2TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c2tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2TBR)

For information about available fields see [`mod@mdma_c2tbr`]
module*/
pub type MDMA_C2TBR = crate::Reg<mdma_c2tbr::MDMA_C2TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c2tbr;
/**MDMA_C2MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c2mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2MAR)

For information about available fields see [`mod@mdma_c2mar`]
module*/
pub type MDMA_C2MAR = crate::Reg<mdma_c2mar::MDMA_C2MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c2mar;
/**MDMA_C2MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c2mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C2MDR)

For information about available fields see [`mod@mdma_c2mdr`]
module*/
pub type MDMA_C2MDR = crate::Reg<mdma_c2mdr::MDMA_C2MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c2mdr;
/**MDMA_C3ISR (r) register accessor: MDMA channel 3 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c3isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3ISR)

For information about available fields see [`mod@mdma_c3isr`]
module*/
pub type MDMA_C3ISR = crate::Reg<mdma_c3isr::MDMA_C3ISRrs>;
///MDMA channel 3 interrupt/status register
pub mod mdma_c3isr;
/**MDMA_C3IFCR (w) register accessor: MDMA channel 3 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3IFCR)

For information about available fields see [`mod@mdma_c3ifcr`]
module*/
pub type MDMA_C3IFCR = crate::Reg<mdma_c3ifcr::MDMA_C3IFCRrs>;
///MDMA channel 3 interrupt flag clear register
pub mod mdma_c3ifcr;
/**MDMA_C3ESR (r) register accessor: MDMA channel 3 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c3esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3ESR)

For information about available fields see [`mod@mdma_c3esr`]
module*/
pub type MDMA_C3ESR = crate::Reg<mdma_c3esr::MDMA_C3ESRrs>;
///MDMA channel 3 error status register
pub mod mdma_c3esr;
/**MDMA_C3CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3CR)

For information about available fields see [`mod@mdma_c3cr`]
module*/
pub type MDMA_C3CR = crate::Reg<mdma_c3cr::MDMA_C3CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c3cr;
/**MDMA_C3TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c3tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3TCR)

For information about available fields see [`mod@mdma_c3tcr`]
module*/
pub type MDMA_C3TCR = crate::Reg<mdma_c3tcr::MDMA_C3TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c3tcr;
/**MDMA_C3BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c3bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3BNDTR)

For information about available fields see [`mod@mdma_c3bndtr`]
module*/
pub type MDMA_C3BNDTR = crate::Reg<mdma_c3bndtr::MDMA_C3BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c3bndtr;
/**MDMA_C3SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c3sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3SAR)

For information about available fields see [`mod@mdma_c3sar`]
module*/
pub type MDMA_C3SAR = crate::Reg<mdma_c3sar::MDMA_C3SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c3sar;
/**MDMA_C3DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c3dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3DAR)

For information about available fields see [`mod@mdma_c3dar`]
module*/
pub type MDMA_C3DAR = crate::Reg<mdma_c3dar::MDMA_C3DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c3dar;
/**MDMA_C3BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c3brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3BRUR)

For information about available fields see [`mod@mdma_c3brur`]
module*/
pub type MDMA_C3BRUR = crate::Reg<mdma_c3brur::MDMA_C3BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c3brur;
/**MDMA_C3LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c3lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3LAR)

For information about available fields see [`mod@mdma_c3lar`]
module*/
pub type MDMA_C3LAR = crate::Reg<mdma_c3lar::MDMA_C3LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c3lar;
/**MDMA_C3TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c3tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3TBR)

For information about available fields see [`mod@mdma_c3tbr`]
module*/
pub type MDMA_C3TBR = crate::Reg<mdma_c3tbr::MDMA_C3TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c3tbr;
/**MDMA_C3MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c3mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3MAR)

For information about available fields see [`mod@mdma_c3mar`]
module*/
pub type MDMA_C3MAR = crate::Reg<mdma_c3mar::MDMA_C3MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c3mar;
/**MDMA_C3MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c3mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C3MDR)

For information about available fields see [`mod@mdma_c3mdr`]
module*/
pub type MDMA_C3MDR = crate::Reg<mdma_c3mdr::MDMA_C3MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c3mdr;
/**MDMA_C4ISR (r) register accessor: MDMA channel 4 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c4isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4ISR)

For information about available fields see [`mod@mdma_c4isr`]
module*/
pub type MDMA_C4ISR = crate::Reg<mdma_c4isr::MDMA_C4ISRrs>;
///MDMA channel 4 interrupt/status register
pub mod mdma_c4isr;
/**MDMA_C4IFCR (w) register accessor: MDMA channel 4 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4IFCR)

For information about available fields see [`mod@mdma_c4ifcr`]
module*/
pub type MDMA_C4IFCR = crate::Reg<mdma_c4ifcr::MDMA_C4IFCRrs>;
///MDMA channel 4 interrupt flag clear register
pub mod mdma_c4ifcr;
/**MDMA_C4ESR (r) register accessor: MDMA channel 4 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c4esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4ESR)

For information about available fields see [`mod@mdma_c4esr`]
module*/
pub type MDMA_C4ESR = crate::Reg<mdma_c4esr::MDMA_C4ESRrs>;
///MDMA channel 4 error status register
pub mod mdma_c4esr;
/**MDMA_C4CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c4cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4CR)

For information about available fields see [`mod@mdma_c4cr`]
module*/
pub type MDMA_C4CR = crate::Reg<mdma_c4cr::MDMA_C4CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c4cr;
/**MDMA_C4TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c4tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4TCR)

For information about available fields see [`mod@mdma_c4tcr`]
module*/
pub type MDMA_C4TCR = crate::Reg<mdma_c4tcr::MDMA_C4TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c4tcr;
/**MDMA_C4BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c4bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4BNDTR)

For information about available fields see [`mod@mdma_c4bndtr`]
module*/
pub type MDMA_C4BNDTR = crate::Reg<mdma_c4bndtr::MDMA_C4BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c4bndtr;
/**MDMA_C4SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c4sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4SAR)

For information about available fields see [`mod@mdma_c4sar`]
module*/
pub type MDMA_C4SAR = crate::Reg<mdma_c4sar::MDMA_C4SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c4sar;
/**MDMA_C4DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c4dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4DAR)

For information about available fields see [`mod@mdma_c4dar`]
module*/
pub type MDMA_C4DAR = crate::Reg<mdma_c4dar::MDMA_C4DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c4dar;
/**MDMA_C4BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c4brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4BRUR)

For information about available fields see [`mod@mdma_c4brur`]
module*/
pub type MDMA_C4BRUR = crate::Reg<mdma_c4brur::MDMA_C4BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c4brur;
/**MDMA_C4LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c4lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4LAR)

For information about available fields see [`mod@mdma_c4lar`]
module*/
pub type MDMA_C4LAR = crate::Reg<mdma_c4lar::MDMA_C4LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c4lar;
/**MDMA_C4TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c4tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4TBR)

For information about available fields see [`mod@mdma_c4tbr`]
module*/
pub type MDMA_C4TBR = crate::Reg<mdma_c4tbr::MDMA_C4TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c4tbr;
/**MDMA_C4MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c4mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4MAR)

For information about available fields see [`mod@mdma_c4mar`]
module*/
pub type MDMA_C4MAR = crate::Reg<mdma_c4mar::MDMA_C4MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c4mar;
/**MDMA_C4MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c4mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c4mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C4MDR)

For information about available fields see [`mod@mdma_c4mdr`]
module*/
pub type MDMA_C4MDR = crate::Reg<mdma_c4mdr::MDMA_C4MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c4mdr;
/**MDMA_C5ISR (r) register accessor: MDMA channel 5 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c5isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5ISR)

For information about available fields see [`mod@mdma_c5isr`]
module*/
pub type MDMA_C5ISR = crate::Reg<mdma_c5isr::MDMA_C5ISRrs>;
///MDMA channel 5 interrupt/status register
pub mod mdma_c5isr;
/**MDMA_C5IFCR (w) register accessor: MDMA channel 5 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5IFCR)

For information about available fields see [`mod@mdma_c5ifcr`]
module*/
pub type MDMA_C5IFCR = crate::Reg<mdma_c5ifcr::MDMA_C5IFCRrs>;
///MDMA channel 5 interrupt flag clear register
pub mod mdma_c5ifcr;
/**MDMA_C5ESR (r) register accessor: MDMA channel 5 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c5esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5ESR)

For information about available fields see [`mod@mdma_c5esr`]
module*/
pub type MDMA_C5ESR = crate::Reg<mdma_c5esr::MDMA_C5ESRrs>;
///MDMA channel 5 error status register
pub mod mdma_c5esr;
/**MDMA_C5CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c5cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5CR)

For information about available fields see [`mod@mdma_c5cr`]
module*/
pub type MDMA_C5CR = crate::Reg<mdma_c5cr::MDMA_C5CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c5cr;
/**MDMA_C5TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c5tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5TCR)

For information about available fields see [`mod@mdma_c5tcr`]
module*/
pub type MDMA_C5TCR = crate::Reg<mdma_c5tcr::MDMA_C5TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c5tcr;
/**MDMA_C5BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c5bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5BNDTR)

For information about available fields see [`mod@mdma_c5bndtr`]
module*/
pub type MDMA_C5BNDTR = crate::Reg<mdma_c5bndtr::MDMA_C5BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c5bndtr;
/**MDMA_C5SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c5sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5SAR)

For information about available fields see [`mod@mdma_c5sar`]
module*/
pub type MDMA_C5SAR = crate::Reg<mdma_c5sar::MDMA_C5SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c5sar;
/**MDMA_C5DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c5dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5DAR)

For information about available fields see [`mod@mdma_c5dar`]
module*/
pub type MDMA_C5DAR = crate::Reg<mdma_c5dar::MDMA_C5DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c5dar;
/**MDMA_C5BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c5brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5BRUR)

For information about available fields see [`mod@mdma_c5brur`]
module*/
pub type MDMA_C5BRUR = crate::Reg<mdma_c5brur::MDMA_C5BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c5brur;
/**MDMA_C5LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c5lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5LAR)

For information about available fields see [`mod@mdma_c5lar`]
module*/
pub type MDMA_C5LAR = crate::Reg<mdma_c5lar::MDMA_C5LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c5lar;
/**MDMA_C5TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c5tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5TBR)

For information about available fields see [`mod@mdma_c5tbr`]
module*/
pub type MDMA_C5TBR = crate::Reg<mdma_c5tbr::MDMA_C5TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c5tbr;
/**MDMA_C5MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c5mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5MAR)

For information about available fields see [`mod@mdma_c5mar`]
module*/
pub type MDMA_C5MAR = crate::Reg<mdma_c5mar::MDMA_C5MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c5mar;
/**MDMA_C5MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c5mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c5mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C5MDR)

For information about available fields see [`mod@mdma_c5mdr`]
module*/
pub type MDMA_C5MDR = crate::Reg<mdma_c5mdr::MDMA_C5MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c5mdr;
/**MDMA_C6ISR (r) register accessor: MDMA channel 6 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c6isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6ISR)

For information about available fields see [`mod@mdma_c6isr`]
module*/
pub type MDMA_C6ISR = crate::Reg<mdma_c6isr::MDMA_C6ISRrs>;
///MDMA channel 6 interrupt/status register
pub mod mdma_c6isr;
/**MDMA_C6IFCR (w) register accessor: MDMA channel 6 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6IFCR)

For information about available fields see [`mod@mdma_c6ifcr`]
module*/
pub type MDMA_C6IFCR = crate::Reg<mdma_c6ifcr::MDMA_C6IFCRrs>;
///MDMA channel 6 interrupt flag clear register
pub mod mdma_c6ifcr;
/**MDMA_C6ESR (r) register accessor: MDMA channel 6 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c6esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6ESR)

For information about available fields see [`mod@mdma_c6esr`]
module*/
pub type MDMA_C6ESR = crate::Reg<mdma_c6esr::MDMA_C6ESRrs>;
///MDMA channel 6 error status register
pub mod mdma_c6esr;
/**MDMA_C6CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c6cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6CR)

For information about available fields see [`mod@mdma_c6cr`]
module*/
pub type MDMA_C6CR = crate::Reg<mdma_c6cr::MDMA_C6CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c6cr;
/**MDMA_C6TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c6tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6TCR)

For information about available fields see [`mod@mdma_c6tcr`]
module*/
pub type MDMA_C6TCR = crate::Reg<mdma_c6tcr::MDMA_C6TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c6tcr;
/**MDMA_C6BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c6bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6BNDTR)

For information about available fields see [`mod@mdma_c6bndtr`]
module*/
pub type MDMA_C6BNDTR = crate::Reg<mdma_c6bndtr::MDMA_C6BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c6bndtr;
/**MDMA_C6SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c6sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6SAR)

For information about available fields see [`mod@mdma_c6sar`]
module*/
pub type MDMA_C6SAR = crate::Reg<mdma_c6sar::MDMA_C6SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c6sar;
/**MDMA_C6DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c6dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6DAR)

For information about available fields see [`mod@mdma_c6dar`]
module*/
pub type MDMA_C6DAR = crate::Reg<mdma_c6dar::MDMA_C6DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c6dar;
/**MDMA_C6BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c6brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6BRUR)

For information about available fields see [`mod@mdma_c6brur`]
module*/
pub type MDMA_C6BRUR = crate::Reg<mdma_c6brur::MDMA_C6BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c6brur;
/**MDMA_C6LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c6lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6LAR)

For information about available fields see [`mod@mdma_c6lar`]
module*/
pub type MDMA_C6LAR = crate::Reg<mdma_c6lar::MDMA_C6LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c6lar;
/**MDMA_C6TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c6tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6TBR)

For information about available fields see [`mod@mdma_c6tbr`]
module*/
pub type MDMA_C6TBR = crate::Reg<mdma_c6tbr::MDMA_C6TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c6tbr;
/**MDMA_C6MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c6mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6MAR)

For information about available fields see [`mod@mdma_c6mar`]
module*/
pub type MDMA_C6MAR = crate::Reg<mdma_c6mar::MDMA_C6MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c6mar;
/**MDMA_C6MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c6mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c6mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C6MDR)

For information about available fields see [`mod@mdma_c6mdr`]
module*/
pub type MDMA_C6MDR = crate::Reg<mdma_c6mdr::MDMA_C6MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c6mdr;
/**MDMA_C7ISR (r) register accessor: MDMA channel 7 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c7isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7ISR)

For information about available fields see [`mod@mdma_c7isr`]
module*/
pub type MDMA_C7ISR = crate::Reg<mdma_c7isr::MDMA_C7ISRrs>;
///MDMA channel 7 interrupt/status register
pub mod mdma_c7isr;
/**MDMA_C7IFCR (w) register accessor: MDMA channel 7 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7IFCR)

For information about available fields see [`mod@mdma_c7ifcr`]
module*/
pub type MDMA_C7IFCR = crate::Reg<mdma_c7ifcr::MDMA_C7IFCRrs>;
///MDMA channel 7 interrupt flag clear register
pub mod mdma_c7ifcr;
/**MDMA_C7ESR (r) register accessor: MDMA channel 7 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c7esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7ESR)

For information about available fields see [`mod@mdma_c7esr`]
module*/
pub type MDMA_C7ESR = crate::Reg<mdma_c7esr::MDMA_C7ESRrs>;
///MDMA channel 7 error status register
pub mod mdma_c7esr;
/**MDMA_C7CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c7cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7CR)

For information about available fields see [`mod@mdma_c7cr`]
module*/
pub type MDMA_C7CR = crate::Reg<mdma_c7cr::MDMA_C7CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c7cr;
/**MDMA_C7TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c7tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7TCR)

For information about available fields see [`mod@mdma_c7tcr`]
module*/
pub type MDMA_C7TCR = crate::Reg<mdma_c7tcr::MDMA_C7TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c7tcr;
/**MDMA_C7BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c7bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7BNDTR)

For information about available fields see [`mod@mdma_c7bndtr`]
module*/
pub type MDMA_C7BNDTR = crate::Reg<mdma_c7bndtr::MDMA_C7BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c7bndtr;
/**MDMA_C7SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c7sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7SAR)

For information about available fields see [`mod@mdma_c7sar`]
module*/
pub type MDMA_C7SAR = crate::Reg<mdma_c7sar::MDMA_C7SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c7sar;
/**MDMA_C7DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c7dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7DAR)

For information about available fields see [`mod@mdma_c7dar`]
module*/
pub type MDMA_C7DAR = crate::Reg<mdma_c7dar::MDMA_C7DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c7dar;
/**MDMA_C7BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c7brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7BRUR)

For information about available fields see [`mod@mdma_c7brur`]
module*/
pub type MDMA_C7BRUR = crate::Reg<mdma_c7brur::MDMA_C7BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c7brur;
/**MDMA_C7LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c7lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7LAR)

For information about available fields see [`mod@mdma_c7lar`]
module*/
pub type MDMA_C7LAR = crate::Reg<mdma_c7lar::MDMA_C7LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c7lar;
/**MDMA_C7TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c7tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7TBR)

For information about available fields see [`mod@mdma_c7tbr`]
module*/
pub type MDMA_C7TBR = crate::Reg<mdma_c7tbr::MDMA_C7TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c7tbr;
/**MDMA_C7MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c7mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7MAR)

For information about available fields see [`mod@mdma_c7mar`]
module*/
pub type MDMA_C7MAR = crate::Reg<mdma_c7mar::MDMA_C7MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c7mar;
/**MDMA_C7MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c7mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c7mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C7MDR)

For information about available fields see [`mod@mdma_c7mdr`]
module*/
pub type MDMA_C7MDR = crate::Reg<mdma_c7mdr::MDMA_C7MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c7mdr;
/**MDMA_C8ISR (r) register accessor: MDMA channel 8 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c8isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8ISR)

For information about available fields see [`mod@mdma_c8isr`]
module*/
pub type MDMA_C8ISR = crate::Reg<mdma_c8isr::MDMA_C8ISRrs>;
///MDMA channel 8 interrupt/status register
pub mod mdma_c8isr;
/**MDMA_C8IFCR (w) register accessor: MDMA channel 8 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8IFCR)

For information about available fields see [`mod@mdma_c8ifcr`]
module*/
pub type MDMA_C8IFCR = crate::Reg<mdma_c8ifcr::MDMA_C8IFCRrs>;
///MDMA channel 8 interrupt flag clear register
pub mod mdma_c8ifcr;
/**MDMA_C8ESR (r) register accessor: MDMA channel 8 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c8esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8ESR)

For information about available fields see [`mod@mdma_c8esr`]
module*/
pub type MDMA_C8ESR = crate::Reg<mdma_c8esr::MDMA_C8ESRrs>;
///MDMA channel 8 error status register
pub mod mdma_c8esr;
/**MDMA_C8CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c8cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8CR)

For information about available fields see [`mod@mdma_c8cr`]
module*/
pub type MDMA_C8CR = crate::Reg<mdma_c8cr::MDMA_C8CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c8cr;
/**MDMA_C8TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c8tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8TCR)

For information about available fields see [`mod@mdma_c8tcr`]
module*/
pub type MDMA_C8TCR = crate::Reg<mdma_c8tcr::MDMA_C8TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c8tcr;
/**MDMA_C8BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c8bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8BNDTR)

For information about available fields see [`mod@mdma_c8bndtr`]
module*/
pub type MDMA_C8BNDTR = crate::Reg<mdma_c8bndtr::MDMA_C8BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c8bndtr;
/**MDMA_C8SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c8sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8SAR)

For information about available fields see [`mod@mdma_c8sar`]
module*/
pub type MDMA_C8SAR = crate::Reg<mdma_c8sar::MDMA_C8SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c8sar;
/**MDMA_C8DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c8dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8DAR)

For information about available fields see [`mod@mdma_c8dar`]
module*/
pub type MDMA_C8DAR = crate::Reg<mdma_c8dar::MDMA_C8DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c8dar;
/**MDMA_C8BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c8brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8BRUR)

For information about available fields see [`mod@mdma_c8brur`]
module*/
pub type MDMA_C8BRUR = crate::Reg<mdma_c8brur::MDMA_C8BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c8brur;
/**MDMA_C8LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c8lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8LAR)

For information about available fields see [`mod@mdma_c8lar`]
module*/
pub type MDMA_C8LAR = crate::Reg<mdma_c8lar::MDMA_C8LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c8lar;
/**MDMA_C8TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c8tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8TBR)

For information about available fields see [`mod@mdma_c8tbr`]
module*/
pub type MDMA_C8TBR = crate::Reg<mdma_c8tbr::MDMA_C8TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c8tbr;
/**MDMA_C8MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c8mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8MAR)

For information about available fields see [`mod@mdma_c8mar`]
module*/
pub type MDMA_C8MAR = crate::Reg<mdma_c8mar::MDMA_C8MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c8mar;
/**MDMA_C8MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c8mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C8MDR)

For information about available fields see [`mod@mdma_c8mdr`]
module*/
pub type MDMA_C8MDR = crate::Reg<mdma_c8mdr::MDMA_C8MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c8mdr;
/**MDMA_C9ISR (r) register accessor: MDMA channel 9 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c9isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9ISR)

For information about available fields see [`mod@mdma_c9isr`]
module*/
pub type MDMA_C9ISR = crate::Reg<mdma_c9isr::MDMA_C9ISRrs>;
///MDMA channel 9 interrupt/status register
pub mod mdma_c9isr;
/**MDMA_C9IFCR (w) register accessor: MDMA channel 9 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9IFCR)

For information about available fields see [`mod@mdma_c9ifcr`]
module*/
pub type MDMA_C9IFCR = crate::Reg<mdma_c9ifcr::MDMA_C9IFCRrs>;
///MDMA channel 9 interrupt flag clear register
pub mod mdma_c9ifcr;
/**MDMA_C9ESR (r) register accessor: MDMA channel 9 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c9esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9ESR)

For information about available fields see [`mod@mdma_c9esr`]
module*/
pub type MDMA_C9ESR = crate::Reg<mdma_c9esr::MDMA_C9ESRrs>;
///MDMA channel 9 error status register
pub mod mdma_c9esr;
/**MDMA_C9CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c9cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9CR)

For information about available fields see [`mod@mdma_c9cr`]
module*/
pub type MDMA_C9CR = crate::Reg<mdma_c9cr::MDMA_C9CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c9cr;
/**MDMA_C9TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c9tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9TCR)

For information about available fields see [`mod@mdma_c9tcr`]
module*/
pub type MDMA_C9TCR = crate::Reg<mdma_c9tcr::MDMA_C9TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c9tcr;
/**MDMA_C9BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c9bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9BNDTR)

For information about available fields see [`mod@mdma_c9bndtr`]
module*/
pub type MDMA_C9BNDTR = crate::Reg<mdma_c9bndtr::MDMA_C9BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c9bndtr;
/**MDMA_C9SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c9sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9SAR)

For information about available fields see [`mod@mdma_c9sar`]
module*/
pub type MDMA_C9SAR = crate::Reg<mdma_c9sar::MDMA_C9SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c9sar;
/**MDMA_C9DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c9dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9DAR)

For information about available fields see [`mod@mdma_c9dar`]
module*/
pub type MDMA_C9DAR = crate::Reg<mdma_c9dar::MDMA_C9DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c9dar;
/**MDMA_C9BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c9brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9BRUR)

For information about available fields see [`mod@mdma_c9brur`]
module*/
pub type MDMA_C9BRUR = crate::Reg<mdma_c9brur::MDMA_C9BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c9brur;
/**MDMA_C9LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c9lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9LAR)

For information about available fields see [`mod@mdma_c9lar`]
module*/
pub type MDMA_C9LAR = crate::Reg<mdma_c9lar::MDMA_C9LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c9lar;
/**MDMA_C9TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c9tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9TBR)

For information about available fields see [`mod@mdma_c9tbr`]
module*/
pub type MDMA_C9TBR = crate::Reg<mdma_c9tbr::MDMA_C9TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c9tbr;
/**MDMA_C9MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c9mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9MAR)

For information about available fields see [`mod@mdma_c9mar`]
module*/
pub type MDMA_C9MAR = crate::Reg<mdma_c9mar::MDMA_C9MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c9mar;
/**MDMA_C9MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c9mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C9MDR)

For information about available fields see [`mod@mdma_c9mdr`]
module*/
pub type MDMA_C9MDR = crate::Reg<mdma_c9mdr::MDMA_C9MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c9mdr;
/**MDMA_C10ISR (r) register accessor: MDMA channel 10 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c10isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10ISR)

For information about available fields see [`mod@mdma_c10isr`]
module*/
pub type MDMA_C10ISR = crate::Reg<mdma_c10isr::MDMA_C10ISRrs>;
///MDMA channel 10 interrupt/status register
pub mod mdma_c10isr;
/**MDMA_C10IFCR (w) register accessor: MDMA channel 10 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10IFCR)

For information about available fields see [`mod@mdma_c10ifcr`]
module*/
pub type MDMA_C10IFCR = crate::Reg<mdma_c10ifcr::MDMA_C10IFCRrs>;
///MDMA channel 10 interrupt flag clear register
pub mod mdma_c10ifcr;
/**MDMA_C10ESR (r) register accessor: MDMA channel 10 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c10esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10ESR)

For information about available fields see [`mod@mdma_c10esr`]
module*/
pub type MDMA_C10ESR = crate::Reg<mdma_c10esr::MDMA_C10ESRrs>;
///MDMA channel 10 error status register
pub mod mdma_c10esr;
/**MDMA_C10CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c10cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10CR)

For information about available fields see [`mod@mdma_c10cr`]
module*/
pub type MDMA_C10CR = crate::Reg<mdma_c10cr::MDMA_C10CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c10cr;
/**MDMA_C10TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c10tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10TCR)

For information about available fields see [`mod@mdma_c10tcr`]
module*/
pub type MDMA_C10TCR = crate::Reg<mdma_c10tcr::MDMA_C10TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c10tcr;
/**MDMA_C10BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c10bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10BNDTR)

For information about available fields see [`mod@mdma_c10bndtr`]
module*/
pub type MDMA_C10BNDTR = crate::Reg<mdma_c10bndtr::MDMA_C10BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c10bndtr;
/**MDMA_C10SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c10sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10SAR)

For information about available fields see [`mod@mdma_c10sar`]
module*/
pub type MDMA_C10SAR = crate::Reg<mdma_c10sar::MDMA_C10SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c10sar;
/**MDMA_C10DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c10dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10DAR)

For information about available fields see [`mod@mdma_c10dar`]
module*/
pub type MDMA_C10DAR = crate::Reg<mdma_c10dar::MDMA_C10DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c10dar;
/**MDMA_C10BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c10brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10BRUR)

For information about available fields see [`mod@mdma_c10brur`]
module*/
pub type MDMA_C10BRUR = crate::Reg<mdma_c10brur::MDMA_C10BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c10brur;
/**MDMA_C10LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c10lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10LAR)

For information about available fields see [`mod@mdma_c10lar`]
module*/
pub type MDMA_C10LAR = crate::Reg<mdma_c10lar::MDMA_C10LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c10lar;
/**MDMA_C10TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c10tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10TBR)

For information about available fields see [`mod@mdma_c10tbr`]
module*/
pub type MDMA_C10TBR = crate::Reg<mdma_c10tbr::MDMA_C10TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c10tbr;
/**MDMA_C10MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c10mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10MAR)

For information about available fields see [`mod@mdma_c10mar`]
module*/
pub type MDMA_C10MAR = crate::Reg<mdma_c10mar::MDMA_C10MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c10mar;
/**MDMA_C10MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c10mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c10mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C10MDR)

For information about available fields see [`mod@mdma_c10mdr`]
module*/
pub type MDMA_C10MDR = crate::Reg<mdma_c10mdr::MDMA_C10MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c10mdr;
/**MDMA_C11ISR (r) register accessor: MDMA channel 11 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c11isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11ISR)

For information about available fields see [`mod@mdma_c11isr`]
module*/
pub type MDMA_C11ISR = crate::Reg<mdma_c11isr::MDMA_C11ISRrs>;
///MDMA channel 11 interrupt/status register
pub mod mdma_c11isr;
/**MDMA_C11IFCR (w) register accessor: MDMA channel 11 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11IFCR)

For information about available fields see [`mod@mdma_c11ifcr`]
module*/
pub type MDMA_C11IFCR = crate::Reg<mdma_c11ifcr::MDMA_C11IFCRrs>;
///MDMA channel 11 interrupt flag clear register
pub mod mdma_c11ifcr;
/**MDMA_C11ESR (r) register accessor: MDMA channel 11 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c11esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11ESR)

For information about available fields see [`mod@mdma_c11esr`]
module*/
pub type MDMA_C11ESR = crate::Reg<mdma_c11esr::MDMA_C11ESRrs>;
///MDMA channel 11 error status register
pub mod mdma_c11esr;
/**MDMA_C11CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c11cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11CR)

For information about available fields see [`mod@mdma_c11cr`]
module*/
pub type MDMA_C11CR = crate::Reg<mdma_c11cr::MDMA_C11CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c11cr;
/**MDMA_C11TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c11tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11TCR)

For information about available fields see [`mod@mdma_c11tcr`]
module*/
pub type MDMA_C11TCR = crate::Reg<mdma_c11tcr::MDMA_C11TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c11tcr;
/**MDMA_C11BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c11bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11BNDTR)

For information about available fields see [`mod@mdma_c11bndtr`]
module*/
pub type MDMA_C11BNDTR = crate::Reg<mdma_c11bndtr::MDMA_C11BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c11bndtr;
/**MDMA_C11SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c11sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11SAR)

For information about available fields see [`mod@mdma_c11sar`]
module*/
pub type MDMA_C11SAR = crate::Reg<mdma_c11sar::MDMA_C11SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c11sar;
/**MDMA_C11DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c11dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11DAR)

For information about available fields see [`mod@mdma_c11dar`]
module*/
pub type MDMA_C11DAR = crate::Reg<mdma_c11dar::MDMA_C11DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c11dar;
/**MDMA_C11BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c11brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11BRUR)

For information about available fields see [`mod@mdma_c11brur`]
module*/
pub type MDMA_C11BRUR = crate::Reg<mdma_c11brur::MDMA_C11BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c11brur;
/**MDMA_C11LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c11lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11LAR)

For information about available fields see [`mod@mdma_c11lar`]
module*/
pub type MDMA_C11LAR = crate::Reg<mdma_c11lar::MDMA_C11LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c11lar;
/**MDMA_C11TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c11tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11TBR)

For information about available fields see [`mod@mdma_c11tbr`]
module*/
pub type MDMA_C11TBR = crate::Reg<mdma_c11tbr::MDMA_C11TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c11tbr;
/**MDMA_C11MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c11mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11MAR)

For information about available fields see [`mod@mdma_c11mar`]
module*/
pub type MDMA_C11MAR = crate::Reg<mdma_c11mar::MDMA_C11MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c11mar;
/**MDMA_C11MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c11mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c11mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C11MDR)

For information about available fields see [`mod@mdma_c11mdr`]
module*/
pub type MDMA_C11MDR = crate::Reg<mdma_c11mdr::MDMA_C11MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c11mdr;
/**MDMA_C12ISR (r) register accessor: MDMA channel 12 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c12isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12ISR)

For information about available fields see [`mod@mdma_c12isr`]
module*/
pub type MDMA_C12ISR = crate::Reg<mdma_c12isr::MDMA_C12ISRrs>;
///MDMA channel 12 interrupt/status register
pub mod mdma_c12isr;
/**MDMA_C12IFCR (w) register accessor: MDMA channel 12 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12IFCR)

For information about available fields see [`mod@mdma_c12ifcr`]
module*/
pub type MDMA_C12IFCR = crate::Reg<mdma_c12ifcr::MDMA_C12IFCRrs>;
///MDMA channel 12 interrupt flag clear register
pub mod mdma_c12ifcr;
/**MDMA_C12ESR (r) register accessor: MDMA channel 12 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c12esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12ESR)

For information about available fields see [`mod@mdma_c12esr`]
module*/
pub type MDMA_C12ESR = crate::Reg<mdma_c12esr::MDMA_C12ESRrs>;
///MDMA channel 12 error status register
pub mod mdma_c12esr;
/**MDMA_C12CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c12cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12CR)

For information about available fields see [`mod@mdma_c12cr`]
module*/
pub type MDMA_C12CR = crate::Reg<mdma_c12cr::MDMA_C12CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c12cr;
/**MDMA_C12TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c12tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12TCR)

For information about available fields see [`mod@mdma_c12tcr`]
module*/
pub type MDMA_C12TCR = crate::Reg<mdma_c12tcr::MDMA_C12TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c12tcr;
/**MDMA_C12BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c12bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12BNDTR)

For information about available fields see [`mod@mdma_c12bndtr`]
module*/
pub type MDMA_C12BNDTR = crate::Reg<mdma_c12bndtr::MDMA_C12BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c12bndtr;
/**MDMA_C12SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c12sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12SAR)

For information about available fields see [`mod@mdma_c12sar`]
module*/
pub type MDMA_C12SAR = crate::Reg<mdma_c12sar::MDMA_C12SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c12sar;
/**MDMA_C12DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c12dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12DAR)

For information about available fields see [`mod@mdma_c12dar`]
module*/
pub type MDMA_C12DAR = crate::Reg<mdma_c12dar::MDMA_C12DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c12dar;
/**MDMA_C12BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c12brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12BRUR)

For information about available fields see [`mod@mdma_c12brur`]
module*/
pub type MDMA_C12BRUR = crate::Reg<mdma_c12brur::MDMA_C12BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c12brur;
/**MDMA_C12LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c12lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12LAR)

For information about available fields see [`mod@mdma_c12lar`]
module*/
pub type MDMA_C12LAR = crate::Reg<mdma_c12lar::MDMA_C12LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c12lar;
/**MDMA_C12TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c12tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12TBR)

For information about available fields see [`mod@mdma_c12tbr`]
module*/
pub type MDMA_C12TBR = crate::Reg<mdma_c12tbr::MDMA_C12TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c12tbr;
/**MDMA_C12MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c12mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12MAR)

For information about available fields see [`mod@mdma_c12mar`]
module*/
pub type MDMA_C12MAR = crate::Reg<mdma_c12mar::MDMA_C12MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c12mar;
/**MDMA_C12MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c12mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c12mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C12MDR)

For information about available fields see [`mod@mdma_c12mdr`]
module*/
pub type MDMA_C12MDR = crate::Reg<mdma_c12mdr::MDMA_C12MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c12mdr;
/**MDMA_C13ISR (r) register accessor: MDMA channel 13 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c13isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13ISR)

For information about available fields see [`mod@mdma_c13isr`]
module*/
pub type MDMA_C13ISR = crate::Reg<mdma_c13isr::MDMA_C13ISRrs>;
///MDMA channel 13 interrupt/status register
pub mod mdma_c13isr;
/**MDMA_C13IFCR (w) register accessor: MDMA channel 13 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13IFCR)

For information about available fields see [`mod@mdma_c13ifcr`]
module*/
pub type MDMA_C13IFCR = crate::Reg<mdma_c13ifcr::MDMA_C13IFCRrs>;
///MDMA channel 13 interrupt flag clear register
pub mod mdma_c13ifcr;
/**MDMA_C13ESR (r) register accessor: MDMA channel 13 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c13esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13ESR)

For information about available fields see [`mod@mdma_c13esr`]
module*/
pub type MDMA_C13ESR = crate::Reg<mdma_c13esr::MDMA_C13ESRrs>;
///MDMA channel 13 error status register
pub mod mdma_c13esr;
/**MDMA_C13CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c13cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13CR)

For information about available fields see [`mod@mdma_c13cr`]
module*/
pub type MDMA_C13CR = crate::Reg<mdma_c13cr::MDMA_C13CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c13cr;
/**MDMA_C13TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c13tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13TCR)

For information about available fields see [`mod@mdma_c13tcr`]
module*/
pub type MDMA_C13TCR = crate::Reg<mdma_c13tcr::MDMA_C13TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c13tcr;
/**MDMA_C13BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c13bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13BNDTR)

For information about available fields see [`mod@mdma_c13bndtr`]
module*/
pub type MDMA_C13BNDTR = crate::Reg<mdma_c13bndtr::MDMA_C13BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c13bndtr;
/**MDMA_C13SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c13sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13SAR)

For information about available fields see [`mod@mdma_c13sar`]
module*/
pub type MDMA_C13SAR = crate::Reg<mdma_c13sar::MDMA_C13SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c13sar;
/**MDMA_C13DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c13dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13DAR)

For information about available fields see [`mod@mdma_c13dar`]
module*/
pub type MDMA_C13DAR = crate::Reg<mdma_c13dar::MDMA_C13DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c13dar;
/**MDMA_C13BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c13brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13BRUR)

For information about available fields see [`mod@mdma_c13brur`]
module*/
pub type MDMA_C13BRUR = crate::Reg<mdma_c13brur::MDMA_C13BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c13brur;
/**MDMA_C13LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c13lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13LAR)

For information about available fields see [`mod@mdma_c13lar`]
module*/
pub type MDMA_C13LAR = crate::Reg<mdma_c13lar::MDMA_C13LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c13lar;
/**MDMA_C13TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c13tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13TBR)

For information about available fields see [`mod@mdma_c13tbr`]
module*/
pub type MDMA_C13TBR = crate::Reg<mdma_c13tbr::MDMA_C13TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c13tbr;
/**MDMA_C13MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c13mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13MAR)

For information about available fields see [`mod@mdma_c13mar`]
module*/
pub type MDMA_C13MAR = crate::Reg<mdma_c13mar::MDMA_C13MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c13mar;
/**MDMA_C13MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c13mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c13mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C13MDR)

For information about available fields see [`mod@mdma_c13mdr`]
module*/
pub type MDMA_C13MDR = crate::Reg<mdma_c13mdr::MDMA_C13MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c13mdr;
/**MDMA_C14ISR (r) register accessor: MDMA channel 14 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c14isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14ISR)

For information about available fields see [`mod@mdma_c14isr`]
module*/
pub type MDMA_C14ISR = crate::Reg<mdma_c14isr::MDMA_C14ISRrs>;
///MDMA channel 14 interrupt/status register
pub mod mdma_c14isr;
/**MDMA_C14IFCR (w) register accessor: MDMA channel 14 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14IFCR)

For information about available fields see [`mod@mdma_c14ifcr`]
module*/
pub type MDMA_C14IFCR = crate::Reg<mdma_c14ifcr::MDMA_C14IFCRrs>;
///MDMA channel 14 interrupt flag clear register
pub mod mdma_c14ifcr;
/**MDMA_C14ESR (r) register accessor: MDMA channel 14 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c14esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14ESR)

For information about available fields see [`mod@mdma_c14esr`]
module*/
pub type MDMA_C14ESR = crate::Reg<mdma_c14esr::MDMA_C14ESRrs>;
///MDMA channel 14 error status register
pub mod mdma_c14esr;
/**MDMA_C14CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c14cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14CR)

For information about available fields see [`mod@mdma_c14cr`]
module*/
pub type MDMA_C14CR = crate::Reg<mdma_c14cr::MDMA_C14CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c14cr;
/**MDMA_C14TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c14tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14TCR)

For information about available fields see [`mod@mdma_c14tcr`]
module*/
pub type MDMA_C14TCR = crate::Reg<mdma_c14tcr::MDMA_C14TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c14tcr;
/**MDMA_C14BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c14bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14BNDTR)

For information about available fields see [`mod@mdma_c14bndtr`]
module*/
pub type MDMA_C14BNDTR = crate::Reg<mdma_c14bndtr::MDMA_C14BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c14bndtr;
/**MDMA_C14SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c14sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14SAR)

For information about available fields see [`mod@mdma_c14sar`]
module*/
pub type MDMA_C14SAR = crate::Reg<mdma_c14sar::MDMA_C14SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c14sar;
/**MDMA_C14DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c14dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14DAR)

For information about available fields see [`mod@mdma_c14dar`]
module*/
pub type MDMA_C14DAR = crate::Reg<mdma_c14dar::MDMA_C14DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c14dar;
/**MDMA_C14BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c14brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14BRUR)

For information about available fields see [`mod@mdma_c14brur`]
module*/
pub type MDMA_C14BRUR = crate::Reg<mdma_c14brur::MDMA_C14BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c14brur;
/**MDMA_C14LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c14lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14LAR)

For information about available fields see [`mod@mdma_c14lar`]
module*/
pub type MDMA_C14LAR = crate::Reg<mdma_c14lar::MDMA_C14LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c14lar;
/**MDMA_C14TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c14tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14TBR)

For information about available fields see [`mod@mdma_c14tbr`]
module*/
pub type MDMA_C14TBR = crate::Reg<mdma_c14tbr::MDMA_C14TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c14tbr;
/**MDMA_C14MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c14mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14MAR)

For information about available fields see [`mod@mdma_c14mar`]
module*/
pub type MDMA_C14MAR = crate::Reg<mdma_c14mar::MDMA_C14MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c14mar;
/**MDMA_C14MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c14mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c14mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C14MDR)

For information about available fields see [`mod@mdma_c14mdr`]
module*/
pub type MDMA_C14MDR = crate::Reg<mdma_c14mdr::MDMA_C14MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c14mdr;
/**MDMA_C15ISR (r) register accessor: MDMA channel 15 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c15isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15ISR)

For information about available fields see [`mod@mdma_c15isr`]
module*/
pub type MDMA_C15ISR = crate::Reg<mdma_c15isr::MDMA_C15ISRrs>;
///MDMA channel 15 interrupt/status register
pub mod mdma_c15isr;
/**MDMA_C15IFCR (w) register accessor: MDMA channel 15 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15IFCR)

For information about available fields see [`mod@mdma_c15ifcr`]
module*/
pub type MDMA_C15IFCR = crate::Reg<mdma_c15ifcr::MDMA_C15IFCRrs>;
///MDMA channel 15 interrupt flag clear register
pub mod mdma_c15ifcr;
/**MDMA_C15ESR (r) register accessor: MDMA channel 15 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c15esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15ESR)

For information about available fields see [`mod@mdma_c15esr`]
module*/
pub type MDMA_C15ESR = crate::Reg<mdma_c15esr::MDMA_C15ESRrs>;
///MDMA channel 15 error status register
pub mod mdma_c15esr;
/**MDMA_C15CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c15cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15CR)

For information about available fields see [`mod@mdma_c15cr`]
module*/
pub type MDMA_C15CR = crate::Reg<mdma_c15cr::MDMA_C15CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c15cr;
/**MDMA_C15TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c15tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15TCR)

For information about available fields see [`mod@mdma_c15tcr`]
module*/
pub type MDMA_C15TCR = crate::Reg<mdma_c15tcr::MDMA_C15TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c15tcr;
/**MDMA_C15BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c15bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15BNDTR)

For information about available fields see [`mod@mdma_c15bndtr`]
module*/
pub type MDMA_C15BNDTR = crate::Reg<mdma_c15bndtr::MDMA_C15BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c15bndtr;
/**MDMA_C15SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c15sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15SAR)

For information about available fields see [`mod@mdma_c15sar`]
module*/
pub type MDMA_C15SAR = crate::Reg<mdma_c15sar::MDMA_C15SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c15sar;
/**MDMA_C15DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c15dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15DAR)

For information about available fields see [`mod@mdma_c15dar`]
module*/
pub type MDMA_C15DAR = crate::Reg<mdma_c15dar::MDMA_C15DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c15dar;
/**MDMA_C15BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c15brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15BRUR)

For information about available fields see [`mod@mdma_c15brur`]
module*/
pub type MDMA_C15BRUR = crate::Reg<mdma_c15brur::MDMA_C15BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c15brur;
/**MDMA_C15LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c15lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15LAR)

For information about available fields see [`mod@mdma_c15lar`]
module*/
pub type MDMA_C15LAR = crate::Reg<mdma_c15lar::MDMA_C15LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c15lar;
/**MDMA_C15TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c15tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15TBR)

For information about available fields see [`mod@mdma_c15tbr`]
module*/
pub type MDMA_C15TBR = crate::Reg<mdma_c15tbr::MDMA_C15TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c15tbr;
/**MDMA_C15MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c15mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15MAR)

For information about available fields see [`mod@mdma_c15mar`]
module*/
pub type MDMA_C15MAR = crate::Reg<mdma_c15mar::MDMA_C15MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c15mar;
/**MDMA_C15MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c15mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C15MDR)

For information about available fields see [`mod@mdma_c15mdr`]
module*/
pub type MDMA_C15MDR = crate::Reg<mdma_c15mdr::MDMA_C15MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c15mdr;
/**MDMA_C16ISR (r) register accessor: MDMA channel 16 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c16isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16ISR)

For information about available fields see [`mod@mdma_c16isr`]
module*/
pub type MDMA_C16ISR = crate::Reg<mdma_c16isr::MDMA_C16ISRrs>;
///MDMA channel 16 interrupt/status register
pub mod mdma_c16isr;
/**MDMA_C16IFCR (w) register accessor: MDMA channel 16 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16IFCR)

For information about available fields see [`mod@mdma_c16ifcr`]
module*/
pub type MDMA_C16IFCR = crate::Reg<mdma_c16ifcr::MDMA_C16IFCRrs>;
///MDMA channel 16 interrupt flag clear register
pub mod mdma_c16ifcr;
/**MDMA_C16ESR (r) register accessor: MDMA channel 16 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c16esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16ESR)

For information about available fields see [`mod@mdma_c16esr`]
module*/
pub type MDMA_C16ESR = crate::Reg<mdma_c16esr::MDMA_C16ESRrs>;
///MDMA channel 16 error status register
pub mod mdma_c16esr;
/**MDMA_C16CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c16cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16CR)

For information about available fields see [`mod@mdma_c16cr`]
module*/
pub type MDMA_C16CR = crate::Reg<mdma_c16cr::MDMA_C16CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c16cr;
/**MDMA_C16TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c16tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16TCR)

For information about available fields see [`mod@mdma_c16tcr`]
module*/
pub type MDMA_C16TCR = crate::Reg<mdma_c16tcr::MDMA_C16TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c16tcr;
/**MDMA_C16BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c16bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16BNDTR)

For information about available fields see [`mod@mdma_c16bndtr`]
module*/
pub type MDMA_C16BNDTR = crate::Reg<mdma_c16bndtr::MDMA_C16BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c16bndtr;
/**MDMA_C16SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c16sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16SAR)

For information about available fields see [`mod@mdma_c16sar`]
module*/
pub type MDMA_C16SAR = crate::Reg<mdma_c16sar::MDMA_C16SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c16sar;
/**MDMA_C16DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c16dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16DAR)

For information about available fields see [`mod@mdma_c16dar`]
module*/
pub type MDMA_C16DAR = crate::Reg<mdma_c16dar::MDMA_C16DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c16dar;
/**MDMA_C16BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c16brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16BRUR)

For information about available fields see [`mod@mdma_c16brur`]
module*/
pub type MDMA_C16BRUR = crate::Reg<mdma_c16brur::MDMA_C16BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c16brur;
/**MDMA_C16LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c16lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16LAR)

For information about available fields see [`mod@mdma_c16lar`]
module*/
pub type MDMA_C16LAR = crate::Reg<mdma_c16lar::MDMA_C16LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c16lar;
/**MDMA_C16TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c16tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16TBR)

For information about available fields see [`mod@mdma_c16tbr`]
module*/
pub type MDMA_C16TBR = crate::Reg<mdma_c16tbr::MDMA_C16TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c16tbr;
/**MDMA_C16MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c16mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16MAR)

For information about available fields see [`mod@mdma_c16mar`]
module*/
pub type MDMA_C16MAR = crate::Reg<mdma_c16mar::MDMA_C16MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c16mar;
/**MDMA_C16MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c16mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c16mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C16MDR)

For information about available fields see [`mod@mdma_c16mdr`]
module*/
pub type MDMA_C16MDR = crate::Reg<mdma_c16mdr::MDMA_C16MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c16mdr;
/**MDMA_C17ISR (r) register accessor: MDMA channel 17 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c17isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17ISR)

For information about available fields see [`mod@mdma_c17isr`]
module*/
pub type MDMA_C17ISR = crate::Reg<mdma_c17isr::MDMA_C17ISRrs>;
///MDMA channel 17 interrupt/status register
pub mod mdma_c17isr;
/**MDMA_C17IFCR (w) register accessor: MDMA channel 17 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17IFCR)

For information about available fields see [`mod@mdma_c17ifcr`]
module*/
pub type MDMA_C17IFCR = crate::Reg<mdma_c17ifcr::MDMA_C17IFCRrs>;
///MDMA channel 17 interrupt flag clear register
pub mod mdma_c17ifcr;
/**MDMA_C17ESR (r) register accessor: MDMA channel 17 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c17esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17ESR)

For information about available fields see [`mod@mdma_c17esr`]
module*/
pub type MDMA_C17ESR = crate::Reg<mdma_c17esr::MDMA_C17ESRrs>;
///MDMA channel 17 error status register
pub mod mdma_c17esr;
/**MDMA_C17CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c17cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17CR)

For information about available fields see [`mod@mdma_c17cr`]
module*/
pub type MDMA_C17CR = crate::Reg<mdma_c17cr::MDMA_C17CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c17cr;
/**MDMA_C17TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c17tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17TCR)

For information about available fields see [`mod@mdma_c17tcr`]
module*/
pub type MDMA_C17TCR = crate::Reg<mdma_c17tcr::MDMA_C17TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c17tcr;
/**MDMA_C17BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c17bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17BNDTR)

For information about available fields see [`mod@mdma_c17bndtr`]
module*/
pub type MDMA_C17BNDTR = crate::Reg<mdma_c17bndtr::MDMA_C17BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c17bndtr;
/**MDMA_C17SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c17sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17SAR)

For information about available fields see [`mod@mdma_c17sar`]
module*/
pub type MDMA_C17SAR = crate::Reg<mdma_c17sar::MDMA_C17SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c17sar;
/**MDMA_C17DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c17dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17DAR)

For information about available fields see [`mod@mdma_c17dar`]
module*/
pub type MDMA_C17DAR = crate::Reg<mdma_c17dar::MDMA_C17DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c17dar;
/**MDMA_C17BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c17brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17BRUR)

For information about available fields see [`mod@mdma_c17brur`]
module*/
pub type MDMA_C17BRUR = crate::Reg<mdma_c17brur::MDMA_C17BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c17brur;
/**MDMA_C17LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c17lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17LAR)

For information about available fields see [`mod@mdma_c17lar`]
module*/
pub type MDMA_C17LAR = crate::Reg<mdma_c17lar::MDMA_C17LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c17lar;
/**MDMA_C17TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c17tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17TBR)

For information about available fields see [`mod@mdma_c17tbr`]
module*/
pub type MDMA_C17TBR = crate::Reg<mdma_c17tbr::MDMA_C17TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c17tbr;
/**MDMA_C17MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c17mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17MAR)

For information about available fields see [`mod@mdma_c17mar`]
module*/
pub type MDMA_C17MAR = crate::Reg<mdma_c17mar::MDMA_C17MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c17mar;
/**MDMA_C17MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c17mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c17mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C17MDR)

For information about available fields see [`mod@mdma_c17mdr`]
module*/
pub type MDMA_C17MDR = crate::Reg<mdma_c17mdr::MDMA_C17MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c17mdr;
/**MDMA_C18ISR (r) register accessor: MDMA channel 18 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c18isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18ISR)

For information about available fields see [`mod@mdma_c18isr`]
module*/
pub type MDMA_C18ISR = crate::Reg<mdma_c18isr::MDMA_C18ISRrs>;
///MDMA channel 18 interrupt/status register
pub mod mdma_c18isr;
/**MDMA_C18IFCR (w) register accessor: MDMA channel 18 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18IFCR)

For information about available fields see [`mod@mdma_c18ifcr`]
module*/
pub type MDMA_C18IFCR = crate::Reg<mdma_c18ifcr::MDMA_C18IFCRrs>;
///MDMA channel 18 interrupt flag clear register
pub mod mdma_c18ifcr;
/**MDMA_C18ESR (r) register accessor: MDMA channel 18 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c18esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18ESR)

For information about available fields see [`mod@mdma_c18esr`]
module*/
pub type MDMA_C18ESR = crate::Reg<mdma_c18esr::MDMA_C18ESRrs>;
///MDMA channel 18 error status register
pub mod mdma_c18esr;
/**MDMA_C18CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c18cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18CR)

For information about available fields see [`mod@mdma_c18cr`]
module*/
pub type MDMA_C18CR = crate::Reg<mdma_c18cr::MDMA_C18CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c18cr;
/**MDMA_C18TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c18tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18TCR)

For information about available fields see [`mod@mdma_c18tcr`]
module*/
pub type MDMA_C18TCR = crate::Reg<mdma_c18tcr::MDMA_C18TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c18tcr;
/**MDMA_C18BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c18bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18BNDTR)

For information about available fields see [`mod@mdma_c18bndtr`]
module*/
pub type MDMA_C18BNDTR = crate::Reg<mdma_c18bndtr::MDMA_C18BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c18bndtr;
/**MDMA_C18SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c18sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18SAR)

For information about available fields see [`mod@mdma_c18sar`]
module*/
pub type MDMA_C18SAR = crate::Reg<mdma_c18sar::MDMA_C18SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c18sar;
/**MDMA_C18DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c18dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18DAR)

For information about available fields see [`mod@mdma_c18dar`]
module*/
pub type MDMA_C18DAR = crate::Reg<mdma_c18dar::MDMA_C18DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c18dar;
/**MDMA_C18BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c18brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18BRUR)

For information about available fields see [`mod@mdma_c18brur`]
module*/
pub type MDMA_C18BRUR = crate::Reg<mdma_c18brur::MDMA_C18BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c18brur;
/**MDMA_C18LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c18lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18LAR)

For information about available fields see [`mod@mdma_c18lar`]
module*/
pub type MDMA_C18LAR = crate::Reg<mdma_c18lar::MDMA_C18LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c18lar;
/**MDMA_C18TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c18tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18TBR)

For information about available fields see [`mod@mdma_c18tbr`]
module*/
pub type MDMA_C18TBR = crate::Reg<mdma_c18tbr::MDMA_C18TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c18tbr;
/**MDMA_C18MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c18mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18MAR)

For information about available fields see [`mod@mdma_c18mar`]
module*/
pub type MDMA_C18MAR = crate::Reg<mdma_c18mar::MDMA_C18MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c18mar;
/**MDMA_C18MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c18mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c18mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C18MDR)

For information about available fields see [`mod@mdma_c18mdr`]
module*/
pub type MDMA_C18MDR = crate::Reg<mdma_c18mdr::MDMA_C18MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c18mdr;
/**MDMA_C19ISR (r) register accessor: MDMA channel 19 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c19isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19ISR)

For information about available fields see [`mod@mdma_c19isr`]
module*/
pub type MDMA_C19ISR = crate::Reg<mdma_c19isr::MDMA_C19ISRrs>;
///MDMA channel 19 interrupt/status register
pub mod mdma_c19isr;
/**MDMA_C19IFCR (w) register accessor: MDMA channel 19 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19IFCR)

For information about available fields see [`mod@mdma_c19ifcr`]
module*/
pub type MDMA_C19IFCR = crate::Reg<mdma_c19ifcr::MDMA_C19IFCRrs>;
///MDMA channel 19 interrupt flag clear register
pub mod mdma_c19ifcr;
/**MDMA_C19ESR (r) register accessor: MDMA channel 19 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c19esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19ESR)

For information about available fields see [`mod@mdma_c19esr`]
module*/
pub type MDMA_C19ESR = crate::Reg<mdma_c19esr::MDMA_C19ESRrs>;
///MDMA channel 19 error status register
pub mod mdma_c19esr;
/**MDMA_C19CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c19cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19CR)

For information about available fields see [`mod@mdma_c19cr`]
module*/
pub type MDMA_C19CR = crate::Reg<mdma_c19cr::MDMA_C19CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c19cr;
/**MDMA_C19TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c19tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19TCR)

For information about available fields see [`mod@mdma_c19tcr`]
module*/
pub type MDMA_C19TCR = crate::Reg<mdma_c19tcr::MDMA_C19TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c19tcr;
/**MDMA_C19BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c19bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19BNDTR)

For information about available fields see [`mod@mdma_c19bndtr`]
module*/
pub type MDMA_C19BNDTR = crate::Reg<mdma_c19bndtr::MDMA_C19BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c19bndtr;
/**MDMA_C19SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c19sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19SAR)

For information about available fields see [`mod@mdma_c19sar`]
module*/
pub type MDMA_C19SAR = crate::Reg<mdma_c19sar::MDMA_C19SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c19sar;
/**MDMA_C19DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c19dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19DAR)

For information about available fields see [`mod@mdma_c19dar`]
module*/
pub type MDMA_C19DAR = crate::Reg<mdma_c19dar::MDMA_C19DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c19dar;
/**MDMA_C19BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c19brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19BRUR)

For information about available fields see [`mod@mdma_c19brur`]
module*/
pub type MDMA_C19BRUR = crate::Reg<mdma_c19brur::MDMA_C19BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c19brur;
/**MDMA_C19LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c19lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19LAR)

For information about available fields see [`mod@mdma_c19lar`]
module*/
pub type MDMA_C19LAR = crate::Reg<mdma_c19lar::MDMA_C19LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c19lar;
/**MDMA_C19TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c19tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19TBR)

For information about available fields see [`mod@mdma_c19tbr`]
module*/
pub type MDMA_C19TBR = crate::Reg<mdma_c19tbr::MDMA_C19TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c19tbr;
/**MDMA_C19MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c19mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19MAR)

For information about available fields see [`mod@mdma_c19mar`]
module*/
pub type MDMA_C19MAR = crate::Reg<mdma_c19mar::MDMA_C19MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c19mar;
/**MDMA_C19MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c19mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c19mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C19MDR)

For information about available fields see [`mod@mdma_c19mdr`]
module*/
pub type MDMA_C19MDR = crate::Reg<mdma_c19mdr::MDMA_C19MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c19mdr;
/**MDMA_C20ISR (r) register accessor: MDMA channel 20 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c20isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20ISR)

For information about available fields see [`mod@mdma_c20isr`]
module*/
pub type MDMA_C20ISR = crate::Reg<mdma_c20isr::MDMA_C20ISRrs>;
///MDMA channel 20 interrupt/status register
pub mod mdma_c20isr;
/**MDMA_C20IFCR (w) register accessor: MDMA channel 20 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20IFCR)

For information about available fields see [`mod@mdma_c20ifcr`]
module*/
pub type MDMA_C20IFCR = crate::Reg<mdma_c20ifcr::MDMA_C20IFCRrs>;
///MDMA channel 20 interrupt flag clear register
pub mod mdma_c20ifcr;
/**MDMA_C20ESR (r) register accessor: MDMA channel 20 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c20esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20ESR)

For information about available fields see [`mod@mdma_c20esr`]
module*/
pub type MDMA_C20ESR = crate::Reg<mdma_c20esr::MDMA_C20ESRrs>;
///MDMA channel 20 error status register
pub mod mdma_c20esr;
/**MDMA_C20CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c20cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20CR)

For information about available fields see [`mod@mdma_c20cr`]
module*/
pub type MDMA_C20CR = crate::Reg<mdma_c20cr::MDMA_C20CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c20cr;
/**MDMA_C20TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c20tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20TCR)

For information about available fields see [`mod@mdma_c20tcr`]
module*/
pub type MDMA_C20TCR = crate::Reg<mdma_c20tcr::MDMA_C20TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c20tcr;
/**MDMA_C20BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c20bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20BNDTR)

For information about available fields see [`mod@mdma_c20bndtr`]
module*/
pub type MDMA_C20BNDTR = crate::Reg<mdma_c20bndtr::MDMA_C20BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c20bndtr;
/**MDMA_C20SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c20sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20SAR)

For information about available fields see [`mod@mdma_c20sar`]
module*/
pub type MDMA_C20SAR = crate::Reg<mdma_c20sar::MDMA_C20SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c20sar;
/**MDMA_C20DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c20dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20DAR)

For information about available fields see [`mod@mdma_c20dar`]
module*/
pub type MDMA_C20DAR = crate::Reg<mdma_c20dar::MDMA_C20DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c20dar;
/**MDMA_C20BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c20brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20BRUR)

For information about available fields see [`mod@mdma_c20brur`]
module*/
pub type MDMA_C20BRUR = crate::Reg<mdma_c20brur::MDMA_C20BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c20brur;
/**MDMA_C20LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c20lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20LAR)

For information about available fields see [`mod@mdma_c20lar`]
module*/
pub type MDMA_C20LAR = crate::Reg<mdma_c20lar::MDMA_C20LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c20lar;
/**MDMA_C20TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c20tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20TBR)

For information about available fields see [`mod@mdma_c20tbr`]
module*/
pub type MDMA_C20TBR = crate::Reg<mdma_c20tbr::MDMA_C20TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c20tbr;
/**MDMA_C20MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c20mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20MAR)

For information about available fields see [`mod@mdma_c20mar`]
module*/
pub type MDMA_C20MAR = crate::Reg<mdma_c20mar::MDMA_C20MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c20mar;
/**MDMA_C20MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c20mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c20mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C20MDR)

For information about available fields see [`mod@mdma_c20mdr`]
module*/
pub type MDMA_C20MDR = crate::Reg<mdma_c20mdr::MDMA_C20MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c20mdr;
/**MDMA_C21ISR (r) register accessor: MDMA channel 21 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c21isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21ISR)

For information about available fields see [`mod@mdma_c21isr`]
module*/
pub type MDMA_C21ISR = crate::Reg<mdma_c21isr::MDMA_C21ISRrs>;
///MDMA channel 21 interrupt/status register
pub mod mdma_c21isr;
/**MDMA_C21IFCR (w) register accessor: MDMA channel 21 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21IFCR)

For information about available fields see [`mod@mdma_c21ifcr`]
module*/
pub type MDMA_C21IFCR = crate::Reg<mdma_c21ifcr::MDMA_C21IFCRrs>;
///MDMA channel 21 interrupt flag clear register
pub mod mdma_c21ifcr;
/**MDMA_C21ESR (r) register accessor: MDMA channel 21 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c21esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21ESR)

For information about available fields see [`mod@mdma_c21esr`]
module*/
pub type MDMA_C21ESR = crate::Reg<mdma_c21esr::MDMA_C21ESRrs>;
///MDMA channel 21 error status register
pub mod mdma_c21esr;
/**MDMA_C21CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c21cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21CR)

For information about available fields see [`mod@mdma_c21cr`]
module*/
pub type MDMA_C21CR = crate::Reg<mdma_c21cr::MDMA_C21CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c21cr;
/**MDMA_C21TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c21tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21TCR)

For information about available fields see [`mod@mdma_c21tcr`]
module*/
pub type MDMA_C21TCR = crate::Reg<mdma_c21tcr::MDMA_C21TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c21tcr;
/**MDMA_C21BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c21bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21BNDTR)

For information about available fields see [`mod@mdma_c21bndtr`]
module*/
pub type MDMA_C21BNDTR = crate::Reg<mdma_c21bndtr::MDMA_C21BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c21bndtr;
/**MDMA_C21SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c21sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21SAR)

For information about available fields see [`mod@mdma_c21sar`]
module*/
pub type MDMA_C21SAR = crate::Reg<mdma_c21sar::MDMA_C21SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c21sar;
/**MDMA_C21DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c21dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21DAR)

For information about available fields see [`mod@mdma_c21dar`]
module*/
pub type MDMA_C21DAR = crate::Reg<mdma_c21dar::MDMA_C21DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c21dar;
/**MDMA_C21BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c21brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21BRUR)

For information about available fields see [`mod@mdma_c21brur`]
module*/
pub type MDMA_C21BRUR = crate::Reg<mdma_c21brur::MDMA_C21BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c21brur;
/**MDMA_C21LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c21lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21LAR)

For information about available fields see [`mod@mdma_c21lar`]
module*/
pub type MDMA_C21LAR = crate::Reg<mdma_c21lar::MDMA_C21LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c21lar;
/**MDMA_C21TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c21tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21TBR)

For information about available fields see [`mod@mdma_c21tbr`]
module*/
pub type MDMA_C21TBR = crate::Reg<mdma_c21tbr::MDMA_C21TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c21tbr;
/**MDMA_C21MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c21mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21MAR)

For information about available fields see [`mod@mdma_c21mar`]
module*/
pub type MDMA_C21MAR = crate::Reg<mdma_c21mar::MDMA_C21MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c21mar;
/**MDMA_C21MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c21mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c21mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C21MDR)

For information about available fields see [`mod@mdma_c21mdr`]
module*/
pub type MDMA_C21MDR = crate::Reg<mdma_c21mdr::MDMA_C21MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c21mdr;
/**MDMA_C22ISR (r) register accessor: MDMA channel 22 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c22isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22ISR)

For information about available fields see [`mod@mdma_c22isr`]
module*/
pub type MDMA_C22ISR = crate::Reg<mdma_c22isr::MDMA_C22ISRrs>;
///MDMA channel 22 interrupt/status register
pub mod mdma_c22isr;
/**MDMA_C22IFCR (w) register accessor: MDMA channel 22 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22IFCR)

For information about available fields see [`mod@mdma_c22ifcr`]
module*/
pub type MDMA_C22IFCR = crate::Reg<mdma_c22ifcr::MDMA_C22IFCRrs>;
///MDMA channel 22 interrupt flag clear register
pub mod mdma_c22ifcr;
/**MDMA_C22ESR (r) register accessor: MDMA channel 22 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c22esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22ESR)

For information about available fields see [`mod@mdma_c22esr`]
module*/
pub type MDMA_C22ESR = crate::Reg<mdma_c22esr::MDMA_C22ESRrs>;
///MDMA channel 22 error status register
pub mod mdma_c22esr;
/**MDMA_C22CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c22cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22CR)

For information about available fields see [`mod@mdma_c22cr`]
module*/
pub type MDMA_C22CR = crate::Reg<mdma_c22cr::MDMA_C22CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c22cr;
/**MDMA_C22TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c22tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22TCR)

For information about available fields see [`mod@mdma_c22tcr`]
module*/
pub type MDMA_C22TCR = crate::Reg<mdma_c22tcr::MDMA_C22TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c22tcr;
/**MDMA_C22BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c22bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22BNDTR)

For information about available fields see [`mod@mdma_c22bndtr`]
module*/
pub type MDMA_C22BNDTR = crate::Reg<mdma_c22bndtr::MDMA_C22BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c22bndtr;
/**MDMA_C22SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c22sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22SAR)

For information about available fields see [`mod@mdma_c22sar`]
module*/
pub type MDMA_C22SAR = crate::Reg<mdma_c22sar::MDMA_C22SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c22sar;
/**MDMA_C22DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c22dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22DAR)

For information about available fields see [`mod@mdma_c22dar`]
module*/
pub type MDMA_C22DAR = crate::Reg<mdma_c22dar::MDMA_C22DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c22dar;
/**MDMA_C22BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c22brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22BRUR)

For information about available fields see [`mod@mdma_c22brur`]
module*/
pub type MDMA_C22BRUR = crate::Reg<mdma_c22brur::MDMA_C22BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c22brur;
/**MDMA_C22LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c22lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22LAR)

For information about available fields see [`mod@mdma_c22lar`]
module*/
pub type MDMA_C22LAR = crate::Reg<mdma_c22lar::MDMA_C22LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c22lar;
/**MDMA_C22TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c22tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22TBR)

For information about available fields see [`mod@mdma_c22tbr`]
module*/
pub type MDMA_C22TBR = crate::Reg<mdma_c22tbr::MDMA_C22TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c22tbr;
/**MDMA_C22MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c22mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22MAR)

For information about available fields see [`mod@mdma_c22mar`]
module*/
pub type MDMA_C22MAR = crate::Reg<mdma_c22mar::MDMA_C22MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c22mar;
/**MDMA_C22MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c22mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c22mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C22MDR)

For information about available fields see [`mod@mdma_c22mdr`]
module*/
pub type MDMA_C22MDR = crate::Reg<mdma_c22mdr::MDMA_C22MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c22mdr;
/**MDMA_C23ISR (r) register accessor: MDMA channel 23 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c23isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23ISR)

For information about available fields see [`mod@mdma_c23isr`]
module*/
pub type MDMA_C23ISR = crate::Reg<mdma_c23isr::MDMA_C23ISRrs>;
///MDMA channel 23 interrupt/status register
pub mod mdma_c23isr;
/**MDMA_C23IFCR (w) register accessor: MDMA channel 23 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23IFCR)

For information about available fields see [`mod@mdma_c23ifcr`]
module*/
pub type MDMA_C23IFCR = crate::Reg<mdma_c23ifcr::MDMA_C23IFCRrs>;
///MDMA channel 23 interrupt flag clear register
pub mod mdma_c23ifcr;
/**MDMA_C23ESR (r) register accessor: MDMA channel 23 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c23esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23ESR)

For information about available fields see [`mod@mdma_c23esr`]
module*/
pub type MDMA_C23ESR = crate::Reg<mdma_c23esr::MDMA_C23ESRrs>;
///MDMA channel 23 error status register
pub mod mdma_c23esr;
/**MDMA_C23CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c23cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23CR)

For information about available fields see [`mod@mdma_c23cr`]
module*/
pub type MDMA_C23CR = crate::Reg<mdma_c23cr::MDMA_C23CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c23cr;
/**MDMA_C23TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c23tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23TCR)

For information about available fields see [`mod@mdma_c23tcr`]
module*/
pub type MDMA_C23TCR = crate::Reg<mdma_c23tcr::MDMA_C23TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c23tcr;
/**MDMA_C23BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c23bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23BNDTR)

For information about available fields see [`mod@mdma_c23bndtr`]
module*/
pub type MDMA_C23BNDTR = crate::Reg<mdma_c23bndtr::MDMA_C23BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c23bndtr;
/**MDMA_C23SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c23sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23SAR)

For information about available fields see [`mod@mdma_c23sar`]
module*/
pub type MDMA_C23SAR = crate::Reg<mdma_c23sar::MDMA_C23SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c23sar;
/**MDMA_C23DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c23dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23DAR)

For information about available fields see [`mod@mdma_c23dar`]
module*/
pub type MDMA_C23DAR = crate::Reg<mdma_c23dar::MDMA_C23DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c23dar;
/**MDMA_C23BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c23brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23BRUR)

For information about available fields see [`mod@mdma_c23brur`]
module*/
pub type MDMA_C23BRUR = crate::Reg<mdma_c23brur::MDMA_C23BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c23brur;
/**MDMA_C23LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c23lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23LAR)

For information about available fields see [`mod@mdma_c23lar`]
module*/
pub type MDMA_C23LAR = crate::Reg<mdma_c23lar::MDMA_C23LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c23lar;
/**MDMA_C23TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c23tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23TBR)

For information about available fields see [`mod@mdma_c23tbr`]
module*/
pub type MDMA_C23TBR = crate::Reg<mdma_c23tbr::MDMA_C23TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c23tbr;
/**MDMA_C23MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c23mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23MAR)

For information about available fields see [`mod@mdma_c23mar`]
module*/
pub type MDMA_C23MAR = crate::Reg<mdma_c23mar::MDMA_C23MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c23mar;
/**MDMA_C23MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c23mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c23mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C23MDR)

For information about available fields see [`mod@mdma_c23mdr`]
module*/
pub type MDMA_C23MDR = crate::Reg<mdma_c23mdr::MDMA_C23MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c23mdr;
/**MDMA_C24ISR (r) register accessor: MDMA channel 24 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c24isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24ISR)

For information about available fields see [`mod@mdma_c24isr`]
module*/
pub type MDMA_C24ISR = crate::Reg<mdma_c24isr::MDMA_C24ISRrs>;
///MDMA channel 24 interrupt/status register
pub mod mdma_c24isr;
/**MDMA_C24IFCR (w) register accessor: MDMA channel 24 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24IFCR)

For information about available fields see [`mod@mdma_c24ifcr`]
module*/
pub type MDMA_C24IFCR = crate::Reg<mdma_c24ifcr::MDMA_C24IFCRrs>;
///MDMA channel 24 interrupt flag clear register
pub mod mdma_c24ifcr;
/**MDMA_C24ESR (r) register accessor: MDMA channel 24 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c24esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24ESR)

For information about available fields see [`mod@mdma_c24esr`]
module*/
pub type MDMA_C24ESR = crate::Reg<mdma_c24esr::MDMA_C24ESRrs>;
///MDMA channel 24 error status register
pub mod mdma_c24esr;
/**MDMA_C24CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c24cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24CR)

For information about available fields see [`mod@mdma_c24cr`]
module*/
pub type MDMA_C24CR = crate::Reg<mdma_c24cr::MDMA_C24CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c24cr;
/**MDMA_C24TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c24tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24TCR)

For information about available fields see [`mod@mdma_c24tcr`]
module*/
pub type MDMA_C24TCR = crate::Reg<mdma_c24tcr::MDMA_C24TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c24tcr;
/**MDMA_C24BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c24bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24BNDTR)

For information about available fields see [`mod@mdma_c24bndtr`]
module*/
pub type MDMA_C24BNDTR = crate::Reg<mdma_c24bndtr::MDMA_C24BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c24bndtr;
/**MDMA_C24SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c24sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24SAR)

For information about available fields see [`mod@mdma_c24sar`]
module*/
pub type MDMA_C24SAR = crate::Reg<mdma_c24sar::MDMA_C24SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c24sar;
/**MDMA_C24DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c24dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24DAR)

For information about available fields see [`mod@mdma_c24dar`]
module*/
pub type MDMA_C24DAR = crate::Reg<mdma_c24dar::MDMA_C24DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c24dar;
/**MDMA_C24BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c24brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24BRUR)

For information about available fields see [`mod@mdma_c24brur`]
module*/
pub type MDMA_C24BRUR = crate::Reg<mdma_c24brur::MDMA_C24BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c24brur;
/**MDMA_C24LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c24lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24LAR)

For information about available fields see [`mod@mdma_c24lar`]
module*/
pub type MDMA_C24LAR = crate::Reg<mdma_c24lar::MDMA_C24LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c24lar;
/**MDMA_C24TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c24tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24TBR)

For information about available fields see [`mod@mdma_c24tbr`]
module*/
pub type MDMA_C24TBR = crate::Reg<mdma_c24tbr::MDMA_C24TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c24tbr;
/**MDMA_C24MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c24mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24MAR)

For information about available fields see [`mod@mdma_c24mar`]
module*/
pub type MDMA_C24MAR = crate::Reg<mdma_c24mar::MDMA_C24MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c24mar;
/**MDMA_C24MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c24mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c24mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C24MDR)

For information about available fields see [`mod@mdma_c24mdr`]
module*/
pub type MDMA_C24MDR = crate::Reg<mdma_c24mdr::MDMA_C24MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c24mdr;
/**MDMA_C25ISR (r) register accessor: MDMA channel 25 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c25isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25ISR)

For information about available fields see [`mod@mdma_c25isr`]
module*/
pub type MDMA_C25ISR = crate::Reg<mdma_c25isr::MDMA_C25ISRrs>;
///MDMA channel 25 interrupt/status register
pub mod mdma_c25isr;
/**MDMA_C25IFCR (w) register accessor: MDMA channel 25 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25IFCR)

For information about available fields see [`mod@mdma_c25ifcr`]
module*/
pub type MDMA_C25IFCR = crate::Reg<mdma_c25ifcr::MDMA_C25IFCRrs>;
///MDMA channel 25 interrupt flag clear register
pub mod mdma_c25ifcr;
/**MDMA_C25ESR (r) register accessor: MDMA channel 25 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c25esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25ESR)

For information about available fields see [`mod@mdma_c25esr`]
module*/
pub type MDMA_C25ESR = crate::Reg<mdma_c25esr::MDMA_C25ESRrs>;
///MDMA channel 25 error status register
pub mod mdma_c25esr;
/**MDMA_C25CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c25cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25CR)

For information about available fields see [`mod@mdma_c25cr`]
module*/
pub type MDMA_C25CR = crate::Reg<mdma_c25cr::MDMA_C25CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c25cr;
/**MDMA_C25TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c25tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25TCR)

For information about available fields see [`mod@mdma_c25tcr`]
module*/
pub type MDMA_C25TCR = crate::Reg<mdma_c25tcr::MDMA_C25TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c25tcr;
/**MDMA_C25BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c25bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25BNDTR)

For information about available fields see [`mod@mdma_c25bndtr`]
module*/
pub type MDMA_C25BNDTR = crate::Reg<mdma_c25bndtr::MDMA_C25BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c25bndtr;
/**MDMA_C25SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c25sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25SAR)

For information about available fields see [`mod@mdma_c25sar`]
module*/
pub type MDMA_C25SAR = crate::Reg<mdma_c25sar::MDMA_C25SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c25sar;
/**MDMA_C25DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c25dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25DAR)

For information about available fields see [`mod@mdma_c25dar`]
module*/
pub type MDMA_C25DAR = crate::Reg<mdma_c25dar::MDMA_C25DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c25dar;
/**MDMA_C25BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c25brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25BRUR)

For information about available fields see [`mod@mdma_c25brur`]
module*/
pub type MDMA_C25BRUR = crate::Reg<mdma_c25brur::MDMA_C25BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c25brur;
/**MDMA_C25LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c25lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25LAR)

For information about available fields see [`mod@mdma_c25lar`]
module*/
pub type MDMA_C25LAR = crate::Reg<mdma_c25lar::MDMA_C25LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c25lar;
/**MDMA_C25TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c25tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25TBR)

For information about available fields see [`mod@mdma_c25tbr`]
module*/
pub type MDMA_C25TBR = crate::Reg<mdma_c25tbr::MDMA_C25TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c25tbr;
/**MDMA_C25MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c25mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25MAR)

For information about available fields see [`mod@mdma_c25mar`]
module*/
pub type MDMA_C25MAR = crate::Reg<mdma_c25mar::MDMA_C25MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c25mar;
/**MDMA_C25MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c25mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c25mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C25MDR)

For information about available fields see [`mod@mdma_c25mdr`]
module*/
pub type MDMA_C25MDR = crate::Reg<mdma_c25mdr::MDMA_C25MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c25mdr;
/**MDMA_C26ISR (r) register accessor: MDMA channel 26 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c26isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26ISR)

For information about available fields see [`mod@mdma_c26isr`]
module*/
pub type MDMA_C26ISR = crate::Reg<mdma_c26isr::MDMA_C26ISRrs>;
///MDMA channel 26 interrupt/status register
pub mod mdma_c26isr;
/**MDMA_C26IFCR (w) register accessor: MDMA channel 26 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26IFCR)

For information about available fields see [`mod@mdma_c26ifcr`]
module*/
pub type MDMA_C26IFCR = crate::Reg<mdma_c26ifcr::MDMA_C26IFCRrs>;
///MDMA channel 26 interrupt flag clear register
pub mod mdma_c26ifcr;
/**MDMA_C26ESR (r) register accessor: MDMA channel 26 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c26esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26ESR)

For information about available fields see [`mod@mdma_c26esr`]
module*/
pub type MDMA_C26ESR = crate::Reg<mdma_c26esr::MDMA_C26ESRrs>;
///MDMA channel 26 error status register
pub mod mdma_c26esr;
/**MDMA_C26CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c26cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26CR)

For information about available fields see [`mod@mdma_c26cr`]
module*/
pub type MDMA_C26CR = crate::Reg<mdma_c26cr::MDMA_C26CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c26cr;
/**MDMA_C26TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c26tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26TCR)

For information about available fields see [`mod@mdma_c26tcr`]
module*/
pub type MDMA_C26TCR = crate::Reg<mdma_c26tcr::MDMA_C26TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c26tcr;
/**MDMA_C26BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c26bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26BNDTR)

For information about available fields see [`mod@mdma_c26bndtr`]
module*/
pub type MDMA_C26BNDTR = crate::Reg<mdma_c26bndtr::MDMA_C26BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c26bndtr;
/**MDMA_C26SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c26sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26SAR)

For information about available fields see [`mod@mdma_c26sar`]
module*/
pub type MDMA_C26SAR = crate::Reg<mdma_c26sar::MDMA_C26SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c26sar;
/**MDMA_C26DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c26dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26DAR)

For information about available fields see [`mod@mdma_c26dar`]
module*/
pub type MDMA_C26DAR = crate::Reg<mdma_c26dar::MDMA_C26DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c26dar;
/**MDMA_C26BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c26brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26BRUR)

For information about available fields see [`mod@mdma_c26brur`]
module*/
pub type MDMA_C26BRUR = crate::Reg<mdma_c26brur::MDMA_C26BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c26brur;
/**MDMA_C26LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c26lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26LAR)

For information about available fields see [`mod@mdma_c26lar`]
module*/
pub type MDMA_C26LAR = crate::Reg<mdma_c26lar::MDMA_C26LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c26lar;
/**MDMA_C26TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c26tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26TBR)

For information about available fields see [`mod@mdma_c26tbr`]
module*/
pub type MDMA_C26TBR = crate::Reg<mdma_c26tbr::MDMA_C26TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c26tbr;
/**MDMA_C26MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c26mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26MAR)

For information about available fields see [`mod@mdma_c26mar`]
module*/
pub type MDMA_C26MAR = crate::Reg<mdma_c26mar::MDMA_C26MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c26mar;
/**MDMA_C26MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c26mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c26mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C26MDR)

For information about available fields see [`mod@mdma_c26mdr`]
module*/
pub type MDMA_C26MDR = crate::Reg<mdma_c26mdr::MDMA_C26MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c26mdr;
/**MDMA_C27ISR (r) register accessor: MDMA channel 27 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c27isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27ISR)

For information about available fields see [`mod@mdma_c27isr`]
module*/
pub type MDMA_C27ISR = crate::Reg<mdma_c27isr::MDMA_C27ISRrs>;
///MDMA channel 27 interrupt/status register
pub mod mdma_c27isr;
/**MDMA_C27IFCR (w) register accessor: MDMA channel 27 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27IFCR)

For information about available fields see [`mod@mdma_c27ifcr`]
module*/
pub type MDMA_C27IFCR = crate::Reg<mdma_c27ifcr::MDMA_C27IFCRrs>;
///MDMA channel 27 interrupt flag clear register
pub mod mdma_c27ifcr;
/**MDMA_C27ESR (r) register accessor: MDMA channel 27 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c27esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27ESR)

For information about available fields see [`mod@mdma_c27esr`]
module*/
pub type MDMA_C27ESR = crate::Reg<mdma_c27esr::MDMA_C27ESRrs>;
///MDMA channel 27 error status register
pub mod mdma_c27esr;
/**MDMA_C27CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c27cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27CR)

For information about available fields see [`mod@mdma_c27cr`]
module*/
pub type MDMA_C27CR = crate::Reg<mdma_c27cr::MDMA_C27CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c27cr;
/**MDMA_C27TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c27tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27TCR)

For information about available fields see [`mod@mdma_c27tcr`]
module*/
pub type MDMA_C27TCR = crate::Reg<mdma_c27tcr::MDMA_C27TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c27tcr;
/**MDMA_C27BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c27bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27BNDTR)

For information about available fields see [`mod@mdma_c27bndtr`]
module*/
pub type MDMA_C27BNDTR = crate::Reg<mdma_c27bndtr::MDMA_C27BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c27bndtr;
/**MDMA_C27SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c27sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27SAR)

For information about available fields see [`mod@mdma_c27sar`]
module*/
pub type MDMA_C27SAR = crate::Reg<mdma_c27sar::MDMA_C27SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c27sar;
/**MDMA_C27DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c27dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27DAR)

For information about available fields see [`mod@mdma_c27dar`]
module*/
pub type MDMA_C27DAR = crate::Reg<mdma_c27dar::MDMA_C27DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c27dar;
/**MDMA_C27BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c27brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27BRUR)

For information about available fields see [`mod@mdma_c27brur`]
module*/
pub type MDMA_C27BRUR = crate::Reg<mdma_c27brur::MDMA_C27BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c27brur;
/**MDMA_C27LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c27lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27LAR)

For information about available fields see [`mod@mdma_c27lar`]
module*/
pub type MDMA_C27LAR = crate::Reg<mdma_c27lar::MDMA_C27LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c27lar;
/**MDMA_C27TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c27tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27TBR)

For information about available fields see [`mod@mdma_c27tbr`]
module*/
pub type MDMA_C27TBR = crate::Reg<mdma_c27tbr::MDMA_C27TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c27tbr;
/**MDMA_C27MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c27mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27MAR)

For information about available fields see [`mod@mdma_c27mar`]
module*/
pub type MDMA_C27MAR = crate::Reg<mdma_c27mar::MDMA_C27MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c27mar;
/**MDMA_C27MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c27mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c27mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C27MDR)

For information about available fields see [`mod@mdma_c27mdr`]
module*/
pub type MDMA_C27MDR = crate::Reg<mdma_c27mdr::MDMA_C27MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c27mdr;
/**MDMA_C28ISR (r) register accessor: MDMA channel 28 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c28isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28ISR)

For information about available fields see [`mod@mdma_c28isr`]
module*/
pub type MDMA_C28ISR = crate::Reg<mdma_c28isr::MDMA_C28ISRrs>;
///MDMA channel 28 interrupt/status register
pub mod mdma_c28isr;
/**MDMA_C28IFCR (w) register accessor: MDMA channel 28 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28IFCR)

For information about available fields see [`mod@mdma_c28ifcr`]
module*/
pub type MDMA_C28IFCR = crate::Reg<mdma_c28ifcr::MDMA_C28IFCRrs>;
///MDMA channel 28 interrupt flag clear register
pub mod mdma_c28ifcr;
/**MDMA_C28ESR (r) register accessor: MDMA channel 28 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c28esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28ESR)

For information about available fields see [`mod@mdma_c28esr`]
module*/
pub type MDMA_C28ESR = crate::Reg<mdma_c28esr::MDMA_C28ESRrs>;
///MDMA channel 28 error status register
pub mod mdma_c28esr;
/**MDMA_C28CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c28cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28CR)

For information about available fields see [`mod@mdma_c28cr`]
module*/
pub type MDMA_C28CR = crate::Reg<mdma_c28cr::MDMA_C28CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c28cr;
/**MDMA_C28TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c28tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28TCR)

For information about available fields see [`mod@mdma_c28tcr`]
module*/
pub type MDMA_C28TCR = crate::Reg<mdma_c28tcr::MDMA_C28TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c28tcr;
/**MDMA_C28BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c28bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28BNDTR)

For information about available fields see [`mod@mdma_c28bndtr`]
module*/
pub type MDMA_C28BNDTR = crate::Reg<mdma_c28bndtr::MDMA_C28BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c28bndtr;
/**MDMA_C28SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c28sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28SAR)

For information about available fields see [`mod@mdma_c28sar`]
module*/
pub type MDMA_C28SAR = crate::Reg<mdma_c28sar::MDMA_C28SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c28sar;
/**MDMA_C28DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c28dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28DAR)

For information about available fields see [`mod@mdma_c28dar`]
module*/
pub type MDMA_C28DAR = crate::Reg<mdma_c28dar::MDMA_C28DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c28dar;
/**MDMA_C28BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c28brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28BRUR)

For information about available fields see [`mod@mdma_c28brur`]
module*/
pub type MDMA_C28BRUR = crate::Reg<mdma_c28brur::MDMA_C28BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c28brur;
/**MDMA_C28LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c28lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28LAR)

For information about available fields see [`mod@mdma_c28lar`]
module*/
pub type MDMA_C28LAR = crate::Reg<mdma_c28lar::MDMA_C28LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c28lar;
/**MDMA_C28TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c28tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28TBR)

For information about available fields see [`mod@mdma_c28tbr`]
module*/
pub type MDMA_C28TBR = crate::Reg<mdma_c28tbr::MDMA_C28TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c28tbr;
/**MDMA_C28MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c28mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28MAR)

For information about available fields see [`mod@mdma_c28mar`]
module*/
pub type MDMA_C28MAR = crate::Reg<mdma_c28mar::MDMA_C28MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c28mar;
/**MDMA_C28MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c28mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c28mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C28MDR)

For information about available fields see [`mod@mdma_c28mdr`]
module*/
pub type MDMA_C28MDR = crate::Reg<mdma_c28mdr::MDMA_C28MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c28mdr;
/**MDMA_C29ISR (r) register accessor: MDMA channel 29 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c29isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29ISR)

For information about available fields see [`mod@mdma_c29isr`]
module*/
pub type MDMA_C29ISR = crate::Reg<mdma_c29isr::MDMA_C29ISRrs>;
///MDMA channel 29 interrupt/status register
pub mod mdma_c29isr;
/**MDMA_C29IFCR (w) register accessor: MDMA channel 29 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29IFCR)

For information about available fields see [`mod@mdma_c29ifcr`]
module*/
pub type MDMA_C29IFCR = crate::Reg<mdma_c29ifcr::MDMA_C29IFCRrs>;
///MDMA channel 29 interrupt flag clear register
pub mod mdma_c29ifcr;
/**MDMA_C29ESR (r) register accessor: MDMA channel 29 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c29esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29ESR)

For information about available fields see [`mod@mdma_c29esr`]
module*/
pub type MDMA_C29ESR = crate::Reg<mdma_c29esr::MDMA_C29ESRrs>;
///MDMA channel 29 error status register
pub mod mdma_c29esr;
/**MDMA_C29CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c29cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29CR)

For information about available fields see [`mod@mdma_c29cr`]
module*/
pub type MDMA_C29CR = crate::Reg<mdma_c29cr::MDMA_C29CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c29cr;
/**MDMA_C29TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c29tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29TCR)

For information about available fields see [`mod@mdma_c29tcr`]
module*/
pub type MDMA_C29TCR = crate::Reg<mdma_c29tcr::MDMA_C29TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c29tcr;
/**MDMA_C29BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c29bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29BNDTR)

For information about available fields see [`mod@mdma_c29bndtr`]
module*/
pub type MDMA_C29BNDTR = crate::Reg<mdma_c29bndtr::MDMA_C29BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c29bndtr;
/**MDMA_C29SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c29sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29SAR)

For information about available fields see [`mod@mdma_c29sar`]
module*/
pub type MDMA_C29SAR = crate::Reg<mdma_c29sar::MDMA_C29SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c29sar;
/**MDMA_C29DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c29dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29DAR)

For information about available fields see [`mod@mdma_c29dar`]
module*/
pub type MDMA_C29DAR = crate::Reg<mdma_c29dar::MDMA_C29DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c29dar;
/**MDMA_C29BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c29brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29BRUR)

For information about available fields see [`mod@mdma_c29brur`]
module*/
pub type MDMA_C29BRUR = crate::Reg<mdma_c29brur::MDMA_C29BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c29brur;
/**MDMA_C29LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c29lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29LAR)

For information about available fields see [`mod@mdma_c29lar`]
module*/
pub type MDMA_C29LAR = crate::Reg<mdma_c29lar::MDMA_C29LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c29lar;
/**MDMA_C29TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c29tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29TBR)

For information about available fields see [`mod@mdma_c29tbr`]
module*/
pub type MDMA_C29TBR = crate::Reg<mdma_c29tbr::MDMA_C29TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c29tbr;
/**MDMA_C29MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c29mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29MAR)

For information about available fields see [`mod@mdma_c29mar`]
module*/
pub type MDMA_C29MAR = crate::Reg<mdma_c29mar::MDMA_C29MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c29mar;
/**MDMA_C29MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c29mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c29mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C29MDR)

For information about available fields see [`mod@mdma_c29mdr`]
module*/
pub type MDMA_C29MDR = crate::Reg<mdma_c29mdr::MDMA_C29MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c29mdr;
/**MDMA_C30ISR (r) register accessor: MDMA channel 30 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c30isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30ISR)

For information about available fields see [`mod@mdma_c30isr`]
module*/
pub type MDMA_C30ISR = crate::Reg<mdma_c30isr::MDMA_C30ISRrs>;
///MDMA channel 30 interrupt/status register
pub mod mdma_c30isr;
/**MDMA_C30IFCR (w) register accessor: MDMA channel 30 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30IFCR)

For information about available fields see [`mod@mdma_c30ifcr`]
module*/
pub type MDMA_C30IFCR = crate::Reg<mdma_c30ifcr::MDMA_C30IFCRrs>;
///MDMA channel 30 interrupt flag clear register
pub mod mdma_c30ifcr;
/**MDMA_C30ESR (r) register accessor: MDMA channel 30 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c30esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30ESR)

For information about available fields see [`mod@mdma_c30esr`]
module*/
pub type MDMA_C30ESR = crate::Reg<mdma_c30esr::MDMA_C30ESRrs>;
///MDMA channel 30 error status register
pub mod mdma_c30esr;
/**MDMA_C30CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c30cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30CR)

For information about available fields see [`mod@mdma_c30cr`]
module*/
pub type MDMA_C30CR = crate::Reg<mdma_c30cr::MDMA_C30CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c30cr;
/**MDMA_C30TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c30tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30TCR)

For information about available fields see [`mod@mdma_c30tcr`]
module*/
pub type MDMA_C30TCR = crate::Reg<mdma_c30tcr::MDMA_C30TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c30tcr;
/**MDMA_C30BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c30bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30BNDTR)

For information about available fields see [`mod@mdma_c30bndtr`]
module*/
pub type MDMA_C30BNDTR = crate::Reg<mdma_c30bndtr::MDMA_C30BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c30bndtr;
/**MDMA_C30SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c30sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30SAR)

For information about available fields see [`mod@mdma_c30sar`]
module*/
pub type MDMA_C30SAR = crate::Reg<mdma_c30sar::MDMA_C30SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c30sar;
/**MDMA_C30DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c30dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30DAR)

For information about available fields see [`mod@mdma_c30dar`]
module*/
pub type MDMA_C30DAR = crate::Reg<mdma_c30dar::MDMA_C30DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c30dar;
/**MDMA_C30BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c30brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30BRUR)

For information about available fields see [`mod@mdma_c30brur`]
module*/
pub type MDMA_C30BRUR = crate::Reg<mdma_c30brur::MDMA_C30BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c30brur;
/**MDMA_C30LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c30lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30LAR)

For information about available fields see [`mod@mdma_c30lar`]
module*/
pub type MDMA_C30LAR = crate::Reg<mdma_c30lar::MDMA_C30LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c30lar;
/**MDMA_C30TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c30tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30TBR)

For information about available fields see [`mod@mdma_c30tbr`]
module*/
pub type MDMA_C30TBR = crate::Reg<mdma_c30tbr::MDMA_C30TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c30tbr;
/**MDMA_C30MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c30mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30MAR)

For information about available fields see [`mod@mdma_c30mar`]
module*/
pub type MDMA_C30MAR = crate::Reg<mdma_c30mar::MDMA_C30MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c30mar;
/**MDMA_C30MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c30mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c30mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C30MDR)

For information about available fields see [`mod@mdma_c30mdr`]
module*/
pub type MDMA_C30MDR = crate::Reg<mdma_c30mdr::MDMA_C30MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c30mdr;
/**MDMA_C31ISR (r) register accessor: MDMA channel 31 interrupt/status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c31isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31ISR)

For information about available fields see [`mod@mdma_c31isr`]
module*/
pub type MDMA_C31ISR = crate::Reg<mdma_c31isr::MDMA_C31ISRrs>;
///MDMA channel 31 interrupt/status register
pub mod mdma_c31isr;
/**MDMA_C31IFCR (w) register accessor: MDMA channel 31 interrupt flag clear register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31IFCR)

For information about available fields see [`mod@mdma_c31ifcr`]
module*/
pub type MDMA_C31IFCR = crate::Reg<mdma_c31ifcr::MDMA_C31IFCRrs>;
///MDMA channel 31 interrupt flag clear register
pub mod mdma_c31ifcr;
/**MDMA_C31ESR (r) register accessor: MDMA channel 31 error status register

You can [`read`](crate::Reg::read) this register and get [`mdma_c31esr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31ESR)

For information about available fields see [`mod@mdma_c31esr`]
module*/
pub type MDMA_C31ESR = crate::Reg<mdma_c31esr::MDMA_C31ESRrs>;
///MDMA channel 31 error status register
pub mod mdma_c31esr;
/**MDMA_C31CR (rw) register accessor: This register is used to control the concerned channel.

You can [`read`](crate::Reg::read) this register and get [`mdma_c31cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31CR)

For information about available fields see [`mod@mdma_c31cr`]
module*/
pub type MDMA_C31CR = crate::Reg<mdma_c31cr::MDMA_C31CRrs>;
///This register is used to control the concerned channel.
pub mod mdma_c31cr;
/**MDMA_C31TCR (rw) register accessor: This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).

You can [`read`](crate::Reg::read) this register and get [`mdma_c31tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31TCR)

For information about available fields see [`mod@mdma_c31tcr`]
module*/
pub type MDMA_C31TCR = crate::Reg<mdma_c31tcr::MDMA_C31TCRrs>;
/**This register is used to configure the concerned channel. In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x00).*/
pub mod mdma_c31tcr;
/**MDMA_C31BNDTR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).

You can [`read`](crate::Reg::read) this register and get [`mdma_c31bndtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31bndtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31BNDTR)

For information about available fields see [`mod@mdma_c31bndtr`]
module*/
pub type MDMA_C31BNDTR = crate::Reg<mdma_c31bndtr::MDMA_C31BNDTRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x04).*/
pub mod mdma_c31bndtr;
/**MDMA_C31SAR (rw) register accessor: In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).

You can [`read`](crate::Reg::read) this register and get [`mdma_c31sar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31sar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31SAR)

For information about available fields see [`mod@mdma_c31sar`]
module*/
pub type MDMA_C31SAR = crate::Reg<mdma_c31sar::MDMA_C31SARrs>;
/**In Linked List mode, at the end of a Block (single or last Block in repeated Block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x08).*/
pub mod mdma_c31sar;
/**MDMA_C31DAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M

You can [`read`](crate::Reg::read) this register and get [`mdma_c31dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31DAR)

For information about available fields see [`mod@mdma_c31dar`]
module*/
pub type MDMA_C31DAR = crate::Reg<mdma_c31dar::MDMA_C31DARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x0C). M*/
pub mod mdma_c31dar;
/**MDMA_C31BRUR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).

You can [`read`](crate::Reg::read) this register and get [`mdma_c31brur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31brur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31BRUR)

For information about available fields see [`mod@mdma_c31brur`]
module*/
pub type MDMA_C31BRUR = crate::Reg<mdma_c31brur::MDMA_C31BRURrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x10).*/
pub mod mdma_c31brur;
/**MDMA_C31LAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.

You can [`read`](crate::Reg::read) this register and get [`mdma_c31lar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31lar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31LAR)

For information about available fields see [`mod@mdma_c31lar`]
module*/
pub type MDMA_C31LAR = crate::Reg<mdma_c31lar::MDMA_C31LARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x14). The new value is only taken into account after all registers are updated, for the next end of block.*/
pub mod mdma_c31lar;
/**MDMA_C31TBR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).

You can [`read`](crate::Reg::read) this register and get [`mdma_c31tbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31tbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31TBR)

For information about available fields see [`mod@mdma_c31tbr`]
module*/
pub type MDMA_C31TBR = crate::Reg<mdma_c31tbr::MDMA_C31TBRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x18).*/
pub mod mdma_c31tbr;
/**MDMA_C31MAR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).

You can [`read`](crate::Reg::read) this register and get [`mdma_c31mar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31mar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31MAR)

For information about available fields see [`mod@mdma_c31mar`]
module*/
pub type MDMA_C31MAR = crate::Reg<mdma_c31mar::MDMA_C31MARrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x20).*/
pub mod mdma_c31mar;
/**MDMA_C31MDR (rw) register accessor: In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).

You can [`read`](crate::Reg::read) this register and get [`mdma_c31mdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c31mdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#MDMA:MDMA_C31MDR)

For information about available fields see [`mod@mdma_c31mdr`]
module*/
pub type MDMA_C31MDR = crate::Reg<mdma_c31mdr::MDMA_C31MDRrs>;
/**In Linked List mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\[31:0\]
+ 0x24).*/
pub mod mdma_c31mdr;
