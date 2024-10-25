///Register `FDCAN_NBTP` reader
pub type R = crate::R<FDCAN_NBTPrs>;
///Register `FDCAN_NBTP` writer
pub type W = crate::W<FDCAN_NBTPrs>;
///Field `NTSEG2` reader - Nominal Time segment after sample point
pub type NTSEG2_R = crate::FieldReader;
///Field `NTSEG2` writer - Nominal Time segment after sample point
pub type NTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `NTSEG1` reader - Nominal Time segment before sample point
pub type NTSEG1_R = crate::FieldReader;
///Field `NTSEG1` writer - Nominal Time segment before sample point
pub type NTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `NBRP` reader - Bit Rate Prescaler
pub type NBRP_R = crate::FieldReader<u16>;
///Field `NBRP` writer - Bit Rate Prescaler
pub type NBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `NSJW` reader - Nominal (Re)Synchronization Jump Width
pub type NSJW_R = crate::FieldReader;
///Field `NSJW` writer - Nominal (Re)Synchronization Jump Width
pub type NSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Nominal Time segment after sample point
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:15 - Nominal Time segment before sample point
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:24 - Bit Rate Prescaler
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bits 25:31 - Nominal (Re)Synchronization Jump Width
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FDCAN_NBTP")
            .field("nsjw", &self.nsjw())
            .field("nbrp", &self.nbrp())
            .field("ntseg1", &self.ntseg1())
            .field("ntseg2", &self.ntseg2())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Nominal Time segment after sample point
    #[inline(always)]
    #[must_use]
    pub fn ntseg2(&mut self) -> NTSEG2_W<FDCAN_NBTPrs> {
        NTSEG2_W::new(self, 0)
    }
    ///Bits 8:15 - Nominal Time segment before sample point
    #[inline(always)]
    #[must_use]
    pub fn ntseg1(&mut self) -> NTSEG1_W<FDCAN_NBTPrs> {
        NTSEG1_W::new(self, 8)
    }
    ///Bits 16:24 - Bit Rate Prescaler
    #[inline(always)]
    #[must_use]
    pub fn nbrp(&mut self) -> NBRP_W<FDCAN_NBTPrs> {
        NBRP_W::new(self, 16)
    }
    ///Bits 25:31 - Nominal (Re)Synchronization Jump Width
    #[inline(always)]
    #[must_use]
    pub fn nsjw(&mut self) -> NSJW_W<FDCAN_NBTPrs> {
        NSJW_W::new(self, 25)
    }
}
/**FDCAN Nominal Bit Timing and Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`fdcan_nbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_nbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U575.html#FDCAN1_RAM:FDCAN_NBTP)*/
pub struct FDCAN_NBTPrs;
impl crate::RegisterSpec for FDCAN_NBTPrs {
    type Ux = u32;
}
///`read()` method returns [`fdcan_nbtp::R`](R) reader structure
impl crate::Readable for FDCAN_NBTPrs {}
///`write(|w| ..)` method takes [`fdcan_nbtp::W`](W) writer structure
impl crate::Writable for FDCAN_NBTPrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FDCAN_NBTP to value 0x0600_0a03
impl crate::Resettable for FDCAN_NBTPrs {
    const RESET_VALUE: u32 = 0x0600_0a03;
}
