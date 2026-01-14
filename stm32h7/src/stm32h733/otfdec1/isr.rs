///Register `ISR` reader
pub type R = crate::R<ISRrs>;
///Field `SEIF` reader - Security Error Interrupt Flag status This bit is set by hardware and read only by application. Bit is set when at least one security error has been detected (illegal access to keys, illegal write on locked configuration). Bit is cleared when application sets in OTFDEC_ICR the corresponding bit to '1'.
pub type SEIF_R = crate::BitReader;
///Field `XONEIF` reader - Execute-only execute-Never Error Interrupt Flag status This bit is set by hardware and read only by application. Bit is set when a read access and not an instruction fetch is detected on any encrypted region with MODE bits set to 00 or 11. It is also set when an instruction fetch and not a read access is detected on any encrypted region with MODE bits set to 01. Bit is cleared when application sets in OTFDEC_ICR the corresponding bit to '1'.
pub type XONEIF_R = crate::BitReader;
///Field `KEIF` reader - Key Error Interrupt Flag status This bit is set by hardware and read only by application. Bit is set when a read access occurs on any encrypted region following the reset of the key registers by an abort event (tamper detection, unauthorized debugger connection, untrusted boot, RDP level regression). Bit is cleared when application sets in OTFDEC_ICR the corresponding bit to '1'. After KEIF is set any subsequent read to any enabled encrypted region returns a zeroed value. This state remains until OTFDEC keys are initialized again.
pub type KEIF_R = crate::BitReader;
impl R {
    ///Bit 0 - Security Error Interrupt Flag status This bit is set by hardware and read only by application. Bit is set when at least one security error has been detected (illegal access to keys, illegal write on locked configuration). Bit is cleared when application sets in OTFDEC_ICR the corresponding bit to '1'.
    #[inline(always)]
    pub fn seif(&self) -> SEIF_R {
        SEIF_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Execute-only execute-Never Error Interrupt Flag status This bit is set by hardware and read only by application. Bit is set when a read access and not an instruction fetch is detected on any encrypted region with MODE bits set to 00 or 11. It is also set when an instruction fetch and not a read access is detected on any encrypted region with MODE bits set to 01. Bit is cleared when application sets in OTFDEC_ICR the corresponding bit to '1'.
    #[inline(always)]
    pub fn xoneif(&self) -> XONEIF_R {
        XONEIF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Key Error Interrupt Flag status This bit is set by hardware and read only by application. Bit is set when a read access occurs on any encrypted region following the reset of the key registers by an abort event (tamper detection, unauthorized debugger connection, untrusted boot, RDP level regression). Bit is cleared when application sets in OTFDEC_ICR the corresponding bit to '1'. After KEIF is set any subsequent read to any enabled encrypted region returns a zeroed value. This state remains until OTFDEC keys are initialized again.
    #[inline(always)]
    pub fn keif(&self) -> KEIF_R {
        KEIF_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ISR")
            .field("seif", &self.seif())
            .field("xoneif", &self.xoneif())
            .field("keif", &self.keif())
            .finish()
    }
}
/**OTFDEC interrupt status register

You can [`read`](crate::Reg::read) this register and get [`isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H733.html#OTFDEC1:ISR)*/
pub struct ISRrs;
impl crate::RegisterSpec for ISRrs {
    type Ux = u32;
}
///`read()` method returns [`isr::R`](R) reader structure
impl crate::Readable for ISRrs {}
///`reset()` method sets ISR to value 0
impl crate::Resettable for ISRrs {}
