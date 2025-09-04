///Register `PUCR` reader
pub type R = crate::R<PUCRrs>;
///Register `PUCR` writer
pub type W = crate::W<PUCRrs>;
///Field `URCL` reader - ULPS Request on Clock Lane
pub type URCL_R = crate::BitReader;
///Field `URCL` writer - ULPS Request on Clock Lane
pub type URCL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UECL` reader - ULPS Exit on Clock Lane
pub type UECL_R = crate::BitReader;
///Field `UECL` writer - ULPS Exit on Clock Lane
pub type UECL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `URDL` reader - ULPS Request on Data Lane
pub type URDL_R = crate::BitReader;
///Field `URDL` writer - ULPS Request on Data Lane
pub type URDL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UEDL` reader - ULPS Exit on Data Lane
pub type UEDL_R = crate::BitReader;
///Field `UEDL` writer - ULPS Exit on Data Lane
pub type UEDL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - ULPS Request on Clock Lane
    #[inline(always)]
    pub fn urcl(&self) -> URCL_R {
        URCL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - ULPS Exit on Clock Lane
    #[inline(always)]
    pub fn uecl(&self) -> UECL_R {
        UECL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - ULPS Request on Data Lane
    #[inline(always)]
    pub fn urdl(&self) -> URDL_R {
        URDL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ULPS Exit on Data Lane
    #[inline(always)]
    pub fn uedl(&self) -> UEDL_R {
        UEDL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PUCR")
            .field("urcl", &self.urcl())
            .field("uecl", &self.uecl())
            .field("urdl", &self.urdl())
            .field("uedl", &self.uedl())
            .finish()
    }
}
impl W {
    ///Bit 0 - ULPS Request on Clock Lane
    #[inline(always)]
    pub fn urcl(&mut self) -> URCL_W<PUCRrs> {
        URCL_W::new(self, 0)
    }
    ///Bit 1 - ULPS Exit on Clock Lane
    #[inline(always)]
    pub fn uecl(&mut self) -> UECL_W<PUCRrs> {
        UECL_W::new(self, 1)
    }
    ///Bit 2 - ULPS Request on Data Lane
    #[inline(always)]
    pub fn urdl(&mut self) -> URDL_W<PUCRrs> {
        URDL_W::new(self, 2)
    }
    ///Bit 3 - ULPS Exit on Data Lane
    #[inline(always)]
    pub fn uedl(&mut self) -> UEDL_W<PUCRrs> {
        UEDL_W::new(self, 3)
    }
}
/**DSI Host PHY ULPS Control Register

You can [`read`](crate::Reg::read) this register and get [`pucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32F767.html#DSI:PUCR)*/
pub struct PUCRrs;
impl crate::RegisterSpec for PUCRrs {
    type Ux = u32;
}
///`read()` method returns [`pucr::R`](R) reader structure
impl crate::Readable for PUCRrs {}
///`write(|w| ..)` method takes [`pucr::W`](W) writer structure
impl crate::Writable for PUCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets PUCR to value 0
impl crate::Resettable for PUCRrs {}
