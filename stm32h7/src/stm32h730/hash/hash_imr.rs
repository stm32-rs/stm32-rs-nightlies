///Register `HASH_IMR` reader
pub type R = crate::R<HASH_IMRrs>;
///Register `HASH_IMR` writer
pub type W = crate::W<HASH_IMRrs>;
///Field `DINIE` reader - Data input interrupt enable
pub type DINIE_R = crate::BitReader;
///Field `DINIE` writer - Data input interrupt enable
pub type DINIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCIE` reader - Digest calculation completion interrupt enable
pub type DCIE_R = crate::BitReader;
///Field `DCIE` writer - Digest calculation completion interrupt enable
pub type DCIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Data input interrupt enable
    #[inline(always)]
    pub fn dinie(&self) -> DINIE_R {
        DINIE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Digest calculation completion interrupt enable
    #[inline(always)]
    pub fn dcie(&self) -> DCIE_R {
        DCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HASH_IMR")
            .field("dinie", &self.dinie())
            .field("dcie", &self.dcie())
            .finish()
    }
}
impl W {
    ///Bit 0 - Data input interrupt enable
    #[inline(always)]
    pub fn dinie(&mut self) -> DINIE_W<'_, HASH_IMRrs> {
        DINIE_W::new(self, 0)
    }
    ///Bit 1 - Digest calculation completion interrupt enable
    #[inline(always)]
    pub fn dcie(&mut self) -> DCIE_W<'_, HASH_IMRrs> {
        DCIE_W::new(self, 1)
    }
}
/**HASH interrupt enable register

You can [`read`](crate::Reg::read) this register and get [`hash_imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hash_imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H730.html#HASH:HASH_IMR)*/
pub struct HASH_IMRrs;
impl crate::RegisterSpec for HASH_IMRrs {
    type Ux = u32;
}
///`read()` method returns [`hash_imr::R`](R) reader structure
impl crate::Readable for HASH_IMRrs {}
///`write(|w| ..)` method takes [`hash_imr::W`](W) writer structure
impl crate::Writable for HASH_IMRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets HASH_IMR to value 0
impl crate::Resettable for HASH_IMRrs {}
