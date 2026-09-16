///Register `BCR` reader
pub type R = crate::R<BCRrs>;
///Register `BCR` writer
pub type W = crate::W<BCRrs>;
///Field `BCR0` reader - max data speed limitation
pub type BCR0_R = crate::BitReader;
///Field `BCR0` writer - max data speed limitation
pub type BCR0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BCR2` reader - in-band interrupt (IBI) payload
pub type BCR2_R = crate::BitReader;
///Field `BCR2` writer - in-band interrupt (IBI) payload
pub type BCR2_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BCR6` reader - Controller capable
pub type BCR6_R = crate::BitReader;
///Field `BCR6` writer - Controller capable
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
    ///Bit 6 - Controller capable
    #[inline(always)]
    pub fn bcr6(&self) -> BCR6_R {
        BCR6_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BCR")
            .field("bcr0", &self.bcr0())
            .field("bcr2", &self.bcr2())
            .field("bcr6", &self.bcr6())
            .finish()
    }
}
impl W {
    ///Bit 0 - max data speed limitation
    #[inline(always)]
    pub fn bcr0(&mut self) -> BCR0_W<'_, BCRrs> {
        BCR0_W::new(self, 0)
    }
    ///Bit 2 - in-band interrupt (IBI) payload
    #[inline(always)]
    pub fn bcr2(&mut self) -> BCR2_W<'_, BCRrs> {
        BCR2_W::new(self, 2)
    }
    ///Bit 6 - Controller capable
    #[inline(always)]
    pub fn bcr6(&mut self) -> BCR6_W<'_, BCRrs> {
        BCR6_W::new(self, 6)
    }
}
/**I3C bus characteristics register

You can [`read`](crate::Reg::read) this register and get [`bcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#I3C1:BCR)*/
pub struct BCRrs;
impl crate::RegisterSpec for BCRrs {
    type Ux = u32;
}
///`read()` method returns [`bcr::R`](R) reader structure
impl crate::Readable for BCRrs {}
///`write(|w| ..)` method takes [`bcr::W`](W) writer structure
impl crate::Writable for BCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets BCR to value 0
impl crate::Resettable for BCRrs {}
