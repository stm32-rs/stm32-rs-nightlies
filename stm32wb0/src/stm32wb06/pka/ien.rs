///Register `IEN` reader
pub type R = crate::R<IENrs>;
///Register `IEN` writer
pub type W = crate::W<IENrs>;
///Field `READY_EN` reader - READY interrupt enable.
pub type READY_EN_R = crate::BitReader;
///Field `READY_EN` writer - READY interrupt enable.
pub type READY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RAMERR_EN` reader - RAM access error interrupt enable.
pub type RAMERR_EN_R = crate::BitReader;
///Field `RAMERR_EN` writer - RAM access error interrupt enable.
pub type RAMERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ADDERR_EN` reader - AHB Address error interrupt enable.
pub type ADDERR_EN_R = crate::BitReader;
///Field `ADDERR_EN` writer - AHB Address error interrupt enable.
pub type ADDERR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - READY interrupt enable.
    #[inline(always)]
    pub fn ready_en(&self) -> READY_EN_R {
        READY_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 2 - RAM access error interrupt enable.
    #[inline(always)]
    pub fn ramerr_en(&self) -> RAMERR_EN_R {
        RAMERR_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - AHB Address error interrupt enable.
    #[inline(always)]
    pub fn adderr_en(&self) -> ADDERR_EN_R {
        ADDERR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IEN")
            .field("ready_en", &self.ready_en())
            .field("ramerr_en", &self.ramerr_en())
            .field("adderr_en", &self.adderr_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - READY interrupt enable.
    #[inline(always)]
    pub fn ready_en(&mut self) -> READY_EN_W<'_, IENrs> {
        READY_EN_W::new(self, 0)
    }
    ///Bit 2 - RAM access error interrupt enable.
    #[inline(always)]
    pub fn ramerr_en(&mut self) -> RAMERR_EN_W<'_, IENrs> {
        RAMERR_EN_W::new(self, 2)
    }
    ///Bit 3 - AHB Address error interrupt enable.
    #[inline(always)]
    pub fn adderr_en(&mut self) -> ADDERR_EN_W<'_, IENrs> {
        ADDERR_EN_W::new(self, 3)
    }
}
/**PKA_IEN register

You can [`read`](crate::Reg::read) this register and get [`ien::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ien::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB06.html#PKA:IEN)*/
pub struct IENrs;
impl crate::RegisterSpec for IENrs {
    type Ux = u32;
}
///`read()` method returns [`ien::R`](R) reader structure
impl crate::Readable for IENrs {}
///`write(|w| ..)` method takes [`ien::W`](W) writer structure
impl crate::Writable for IENrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets IEN to value 0
impl crate::Resettable for IENrs {}
