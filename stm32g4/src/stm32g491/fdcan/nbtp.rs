///Register `NBTP` reader
pub type R = crate::R<NBTPrs>;
///Register `NBTP` writer
pub type W = crate::W<NBTPrs>;
///Field `NTSEG2` reader - Nominal time segment after sample point
pub type NTSEG2_R = crate::FieldReader;
///Field `NTSEG2` writer - Nominal time segment after sample point
pub type NTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `NTSEG1` reader - Nominal time segment before sample point
pub type NTSEG1_R = crate::FieldReader;
///Field `NTSEG1` writer - Nominal time segment before sample point
pub type NTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `NBRP` reader - Bit rate prescaler
pub type NBRP_R = crate::FieldReader<u16>;
///Field `NBRP` writer - Bit rate prescaler
pub type NBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `NSJW` reader - Nominal (re)synchronization jump width
pub type NSJW_R = crate::FieldReader;
///Field `NSJW` writer - Nominal (re)synchronization jump width
pub type NSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - Nominal time segment after sample point
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:15 - Nominal time segment before sample point
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:24 - Bit rate prescaler
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bits 25:31 - Nominal (re)synchronization jump width
    #[inline(always)]
    pub fn nsjw(&self) -> NSJW_R {
        NSJW_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NBTP")
            .field("ntseg2", &self.ntseg2())
            .field("ntseg1", &self.ntseg1())
            .field("nbrp", &self.nbrp())
            .field("nsjw", &self.nsjw())
            .finish()
    }
}
impl W {
    ///Bits 0:6 - Nominal time segment after sample point
    #[inline(always)]
    pub fn ntseg2(&mut self) -> NTSEG2_W<'_, NBTPrs> {
        NTSEG2_W::new(self, 0)
    }
    ///Bits 8:15 - Nominal time segment before sample point
    #[inline(always)]
    pub fn ntseg1(&mut self) -> NTSEG1_W<'_, NBTPrs> {
        NTSEG1_W::new(self, 8)
    }
    ///Bits 16:24 - Bit rate prescaler
    #[inline(always)]
    pub fn nbrp(&mut self) -> NBRP_W<'_, NBTPrs> {
        NBRP_W::new(self, 16)
    }
    ///Bits 25:31 - Nominal (re)synchronization jump width
    #[inline(always)]
    pub fn nsjw(&mut self) -> NSJW_W<'_, NBTPrs> {
        NSJW_W::new(self, 25)
    }
}
/**FDCAN nominal bit timing and prescaler register

You can [`read`](crate::Reg::read) this register and get [`nbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32G491.html#FDCAN:NBTP)*/
pub struct NBTPrs;
impl crate::RegisterSpec for NBTPrs {
    type Ux = u32;
}
///`read()` method returns [`nbtp::R`](R) reader structure
impl crate::Readable for NBTPrs {}
///`write(|w| ..)` method takes [`nbtp::W`](W) writer structure
impl crate::Writable for NBTPrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets NBTP to value 0x0600_0a03
impl crate::Resettable for NBTPrs {
    const RESET_VALUE: u32 = 0x0600_0a03;
}
