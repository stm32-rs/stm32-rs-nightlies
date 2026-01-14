///Register `COMP_SR` reader
pub type R = crate::R<COMP_SRrs>;
///Field `C1VAL` reader - COMP Channel1 output status bit This bit is read-only. It reflects the current COMP Channel1 output taking into account POLARITY and BLANKING bits effect.
pub type C1VAL_R = crate::BitReader;
///Field `C1IF` reader - COMP Channel1 interrupt flag This bit is set by hardware when the COMP Channel1 output is set This bit is cleared by software writing 1 the CC1IF bit in the COMP_ICFR register.
pub type C1IF_R = crate::BitReader;
impl R {
    ///Bit 0 - COMP Channel1 output status bit This bit is read-only. It reflects the current COMP Channel1 output taking into account POLARITY and BLANKING bits effect.
    #[inline(always)]
    pub fn c1val(&self) -> C1VAL_R {
        C1VAL_R::new((self.bits & 1) != 0)
    }
    ///Bit 16 - COMP Channel1 interrupt flag This bit is set by hardware when the COMP Channel1 output is set This bit is cleared by software writing 1 the CC1IF bit in the COMP_ICFR register.
    #[inline(always)]
    pub fn c1if(&self) -> C1IF_R {
        C1IF_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("COMP_SR")
            .field("c1val", &self.c1val())
            .field("c1if", &self.c1if())
            .finish()
    }
}
/**Comparator status register

You can [`read`](crate::Reg::read) this register and get [`comp_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#COMP:COMP_SR)*/
pub struct COMP_SRrs;
impl crate::RegisterSpec for COMP_SRrs {
    type Ux = u32;
}
///`read()` method returns [`comp_sr::R`](R) reader structure
impl crate::Readable for COMP_SRrs {}
///`reset()` method sets COMP_SR to value 0
impl crate::Resettable for COMP_SRrs {}
