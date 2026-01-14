///Register `NBTP` reader
pub type R = crate::R<NBTPrs>;
///Register `NBTP` writer
pub type W = crate::W<NBTPrs>;
///Field `NTSEG2` reader - NTSEG2
pub type NTSEG2_R = crate::FieldReader;
///Field `NTSEG2` writer - NTSEG2
pub type NTSEG2_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
///Field `NTSEG1` reader - NTSEG1
pub type NTSEG1_R = crate::FieldReader;
///Field `NTSEG1` writer - NTSEG1
pub type NTSEG1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `NBRP` reader - NBRP
pub type NBRP_R = crate::FieldReader<u16>;
///Field `NBRP` writer - NBRP
pub type NBRP_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `NSJW` reader - NSJW
pub type NSJW_R = crate::FieldReader;
///Field `NSJW` writer - NSJW
pub type NSJW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 0:6 - NTSEG2
    #[inline(always)]
    pub fn ntseg2(&self) -> NTSEG2_R {
        NTSEG2_R::new((self.bits & 0x7f) as u8)
    }
    ///Bits 8:15 - NTSEG1
    #[inline(always)]
    pub fn ntseg1(&self) -> NTSEG1_R {
        NTSEG1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:24 - NBRP
    #[inline(always)]
    pub fn nbrp(&self) -> NBRP_R {
        NBRP_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bits 25:31 - NSJW
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
    ///Bits 0:6 - NTSEG2
    #[inline(always)]
    pub fn ntseg2(&mut self) -> NTSEG2_W<'_, NBTPrs> {
        NTSEG2_W::new(self, 0)
    }
    ///Bits 8:15 - NTSEG1
    #[inline(always)]
    pub fn ntseg1(&mut self) -> NTSEG1_W<'_, NBTPrs> {
        NTSEG1_W::new(self, 8)
    }
    ///Bits 16:24 - NBRP
    #[inline(always)]
    pub fn nbrp(&mut self) -> NBRP_W<'_, NBTPrs> {
        NBRP_W::new(self, 16)
    }
    ///Bits 25:31 - NSJW
    #[inline(always)]
    pub fn nsjw(&mut self) -> NSJW_W<'_, NBTPrs> {
        NSJW_W::new(self, 25)
    }
}
/**This register is dedicated to the nominal bit timing used during the arbitration phase.

You can [`read`](crate::Reg::read) this register and get [`nbtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nbtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#FDCAN1:NBTP)*/
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
///`reset()` method sets NBTP to value 0x0a33
impl crate::Resettable for NBTPrs {
    const RESET_VALUE: u32 = 0x0a33;
}
