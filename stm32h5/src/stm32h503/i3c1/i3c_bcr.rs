///Register `I3C_BCR` reader
pub type R = crate::R<I3C_BCRrs>;
///Register `I3C_BCR` writer
pub type W = crate::W<I3C_BCRrs>;
///Field `BCR0` reader - max data speed limitation
pub type BCR0_R = crate::BitReader;
///Field `BCR0` writer - max data speed limitation
pub type BCR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BCR2` reader - in-band interrupt (IBI) payload
pub type BCR2_R = crate::BitReader;
///Field `BCR2` writer - in-band interrupt (IBI) payload
pub type BCR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BCR6` reader - controller capable
pub type BCR6_R = crate::BitReader;
///Field `BCR6` writer - controller capable
pub type BCR6_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - max data speed limitation
    #[inline(always)]
    pub fn bcr0(&self) -> BCR0_R {
        BCR0_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - in-band interrupt (IBI) payload
    #[inline(always)]
    pub fn bcr2(&self) -> BCR2_R {
        BCR2_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 6 - controller capable
    #[inline(always)]
    pub fn bcr6(&self) -> BCR6_R {
        BCR6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I3C_BCR")
            .field("bcr0", &self.bcr0())
            .field("bcr2", &self.bcr2())
            .field("bcr6", &self.bcr6())
            .finish()
    }
}
impl W {
    ///Bit 0 - max data speed limitation
    #[inline(always)]
    #[must_use]
    pub fn bcr0(&mut self) -> BCR0_W<I3C_BCRrs> {
        BCR0_W::new(self, 0)
    }
    ///Bit 2 - in-band interrupt (IBI) payload
    #[inline(always)]
    #[must_use]
    pub fn bcr2(&mut self) -> BCR2_W<I3C_BCRrs> {
        BCR2_W::new(self, 2)
    }
    ///Bit 6 - controller capable
    #[inline(always)]
    #[must_use]
    pub fn bcr6(&mut self) -> BCR6_W<I3C_BCRrs> {
        BCR6_W::new(self, 6)
    }
}
/**I3C bus characteristics register

You can [`read`](crate::Reg::read) this register and get [`i3c_bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3c_bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H503.html#I3C1:I3C_BCR)*/
pub struct I3C_BCRrs;
impl crate::RegisterSpec for I3C_BCRrs {
    type Ux = u32;
}
///`read()` method returns [`i3c_bcr::R`](R) reader structure
impl crate::Readable for I3C_BCRrs {}
///`write(|w| ..)` method takes [`i3c_bcr::W`](W) writer structure
impl crate::Writable for I3C_BCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets I3C_BCR to value 0
impl crate::Resettable for I3C_BCRrs {
    const RESET_VALUE: u32 = 0;
}
