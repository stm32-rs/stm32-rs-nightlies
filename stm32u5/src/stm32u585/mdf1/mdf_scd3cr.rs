///Register `MDF_SCD3CR` reader
pub type R = crate::R<MDF_SCD3CRrs>;
///Register `MDF_SCD3CR` writer
pub type W = crate::W<MDF_SCD3CRrs>;
///Field `SCDEN` reader - Short circuit detector enable Set and cleared by software. - 0: The short circuit detector is disabled, - 1: The short circuit detector is enabled,
pub type SCDEN_R = crate::BitReader;
///Field `SCDEN` writer - Short circuit detector enable Set and cleared by software. - 0: The short circuit detector is disabled, - 1: The short circuit detector is enabled,
pub type SCDEN_W<'a, REG> = crate::BitWriter<'a, REG>;
/**Field `BKSCD` reader - Break signal assignment for short circuit detector Set and cleared by software. BKSCD\[i\]
= 0: Break signal (mdf_break\[i\]) is not assigned to this SCD event BKSCD\[i\]
= 1: Break signal (mdf_break\[i\]) is assigned to this SCD event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.*/
pub type BKSCD_R = crate::FieldReader;
/**Field `BKSCD` writer - Break signal assignment for short circuit detector Set and cleared by software. BKSCD\[i\]
= 0: Break signal (mdf_break\[i\]) is not assigned to this SCD event BKSCD\[i\]
= 1: Break signal (mdf_break\[i\]) is assigned to this SCD event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.*/
pub type BKSCD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SCDT` reader - Short-circuit detector threshold Set and cleared by software. These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given input stream. - 0: 2 consecutive 1 s or 0 s will generate an event, - 1: 2 consecutive 1 s or 0 s will generate an event - 2: 3 consecutive 1 s or 0 s will generate an event, ... - 255: 256 consecutive 1 s or 0 s will generate an event, This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SCDT_R = crate::FieldReader;
///Field `SCDT` writer - Short-circuit detector threshold Set and cleared by software. These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given input stream. - 0: 2 consecutive 1 s or 0 s will generate an event, - 1: 2 consecutive 1 s or 0 s will generate an event - 2: 3 consecutive 1 s or 0 s will generate an event, ... - 255: 256 consecutive 1 s or 0 s will generate an event, This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
pub type SCDT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `SCDACTIVE` reader - SCD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the SCD is effectively enabled (active) or not. The protected fields of this function can only be updated when the SCDACTIVE is set to a , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SCDEN and a transition on SCDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The SCD is not active, and can be configured if needed - 1: The SCD is active, and protected fields cannot be configured.
pub type SCDACTIVE_R = crate::BitReader;
impl R {
    ///Bit 0 - Short circuit detector enable Set and cleared by software. - 0: The short circuit detector is disabled, - 1: The short circuit detector is enabled,
    #[inline(always)]
    pub fn scden(&self) -> SCDEN_R {
        SCDEN_R::new((self.bits & 1) != 0)
    }
    /**Bits 4:7 - Break signal assignment for short circuit detector Set and cleared by software. BKSCD\[i\]
    = 0: Break signal (mdf_break\[i\]) is not assigned to this SCD event BKSCD\[i\]
    = 1: Break signal (mdf_break\[i\]) is assigned to this SCD event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.*/
    #[inline(always)]
    pub fn bkscd(&self) -> BKSCD_R {
        BKSCD_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 12:19 - Short-circuit detector threshold Set and cleared by software. These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given input stream. - 0: 2 consecutive 1 s or 0 s will generate an event, - 1: 2 consecutive 1 s or 0 s will generate an event - 2: 3 consecutive 1 s or 0 s will generate an event, ... - 255: 256 consecutive 1 s or 0 s will generate an event, This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn scdt(&self) -> SCDT_R {
        SCDT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
    ///Bit 31 - SCD Active flag Set and cleared by hardware. This flag must be used by the application in order to check if the SCD is effectively enabled (active) or not. The protected fields of this function can only be updated when the SCDACTIVE is set to a , please refer to Section 1.4.15: Register protection for details. The delay between a transition on SCDEN and a transition on SCDACTIVE is 2 periods of AHB clock and 2 periods of mdf_proc_ck. - 0: The SCD is not active, and can be configured if needed - 1: The SCD is active, and protected fields cannot be configured.
    #[inline(always)]
    pub fn scdactive(&self) -> SCDACTIVE_R {
        SCDACTIVE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MDF_SCD3CR")
            .field("scden", &self.scden())
            .field("bkscd", &self.bkscd())
            .field("scdt", &self.scdt())
            .field("scdactive", &self.scdactive())
            .finish()
    }
}
impl W {
    ///Bit 0 - Short circuit detector enable Set and cleared by software. - 0: The short circuit detector is disabled, - 1: The short circuit detector is enabled,
    #[inline(always)]
    pub fn scden(&mut self) -> SCDEN_W<MDF_SCD3CRrs> {
        SCDEN_W::new(self, 0)
    }
    /**Bits 4:7 - Break signal assignment for short circuit detector Set and cleared by software. BKSCD\[i\]
    = 0: Break signal (mdf_break\[i\]) is not assigned to this SCD event BKSCD\[i\]
    = 1: Break signal (mdf_break\[i\]) is assigned to this SCD event This field can be write-protected, please refer to Section 1.4.15: Register protection for details.*/
    #[inline(always)]
    pub fn bkscd(&mut self) -> BKSCD_W<MDF_SCD3CRrs> {
        BKSCD_W::new(self, 4)
    }
    ///Bits 12:19 - Short-circuit detector threshold Set and cleared by software. These bits are written by software to define the threshold counter for the short-circuit detector. If this value is reached, then a short-circuit detector event occurs on a given input stream. - 0: 2 consecutive 1 s or 0 s will generate an event, - 1: 2 consecutive 1 s or 0 s will generate an event - 2: 3 consecutive 1 s or 0 s will generate an event, ... - 255: 256 consecutive 1 s or 0 s will generate an event, This field can be write-protected, please refer to Section 1.4.15: Register protection for details.
    #[inline(always)]
    pub fn scdt(&mut self) -> SCDT_W<MDF_SCD3CRrs> {
        SCDT_W::new(self, 12)
    }
}
/**This register is used for the adjustment stream delays.

You can [`read`](crate::Reg::read) this register and get [`mdf_scd3cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdf_scd3cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U585.html#MDF1:MDF_SCD3CR)*/
pub struct MDF_SCD3CRrs;
impl crate::RegisterSpec for MDF_SCD3CRrs {
    type Ux = u32;
}
///`read()` method returns [`mdf_scd3cr::R`](R) reader structure
impl crate::Readable for MDF_SCD3CRrs {}
///`write(|w| ..)` method takes [`mdf_scd3cr::W`](W) writer structure
impl crate::Writable for MDF_SCD3CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets MDF_SCD3CR to value 0
impl crate::Resettable for MDF_SCD3CRrs {
    const RESET_VALUE: u32 = 0;
}
