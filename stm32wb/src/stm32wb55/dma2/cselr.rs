///Register `CSELR` reader
pub type R = crate::R<CSELRrs>;
///Register `CSELR` writer
pub type W = crate::W<CSELRrs>;
///Field `C1S` reader - DMA channel 1 selection
pub type C1S_R = crate::FieldReader;
///Field `C1S` writer - DMA channel 1 selection
pub type C1S_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `C2S` reader - DMA channel 2 selection
pub type C2S_R = crate::FieldReader;
///Field `C2S` writer - DMA channel 2 selection
pub type C2S_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `C3S` reader - DMA channel 3 selection
pub type C3S_R = crate::FieldReader;
///Field `C3S` writer - DMA channel 3 selection
pub type C3S_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `C4S` reader - DMA channel 4 selection
pub type C4S_R = crate::FieldReader;
///Field `C4S` writer - DMA channel 4 selection
pub type C4S_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `C5S` reader - DMA channel 5 selection
pub type C5S_R = crate::FieldReader;
///Field `C5S` writer - DMA channel 5 selection
pub type C5S_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `C6S` reader - DMA channel 6 selection
pub type C6S_R = crate::FieldReader;
///Field `C6S` writer - DMA channel 6 selection
pub type C6S_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `C7S` reader - DMA channel 7 selection
pub type C7S_R = crate::FieldReader;
///Field `C7S` writer - DMA channel 7 selection
pub type C7S_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 0:3 - DMA channel 1 selection
    #[inline(always)]
    pub fn c1s(&self) -> C1S_R {
        C1S_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - DMA channel 2 selection
    #[inline(always)]
    pub fn c2s(&self) -> C2S_R {
        C2S_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    ///Bits 8:11 - DMA channel 3 selection
    #[inline(always)]
    pub fn c3s(&self) -> C3S_R {
        C3S_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 12:15 - DMA channel 4 selection
    #[inline(always)]
    pub fn c4s(&self) -> C4S_R {
        C4S_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bits 16:19 - DMA channel 5 selection
    #[inline(always)]
    pub fn c5s(&self) -> C5S_R {
        C5S_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    ///Bits 20:23 - DMA channel 6 selection
    #[inline(always)]
    pub fn c6s(&self) -> C6S_R {
        C6S_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    ///Bits 24:27 - DMA channel 7 selection
    #[inline(always)]
    pub fn c7s(&self) -> C7S_R {
        C7S_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSELR")
            .field("c7s", &self.c7s())
            .field("c6s", &self.c6s())
            .field("c5s", &self.c5s())
            .field("c4s", &self.c4s())
            .field("c3s", &self.c3s())
            .field("c2s", &self.c2s())
            .field("c1s", &self.c1s())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - DMA channel 1 selection
    #[inline(always)]
    pub fn c1s(&mut self) -> C1S_W<CSELRrs> {
        C1S_W::new(self, 0)
    }
    ///Bits 4:7 - DMA channel 2 selection
    #[inline(always)]
    pub fn c2s(&mut self) -> C2S_W<CSELRrs> {
        C2S_W::new(self, 4)
    }
    ///Bits 8:11 - DMA channel 3 selection
    #[inline(always)]
    pub fn c3s(&mut self) -> C3S_W<CSELRrs> {
        C3S_W::new(self, 8)
    }
    ///Bits 12:15 - DMA channel 4 selection
    #[inline(always)]
    pub fn c4s(&mut self) -> C4S_W<CSELRrs> {
        C4S_W::new(self, 12)
    }
    ///Bits 16:19 - DMA channel 5 selection
    #[inline(always)]
    pub fn c5s(&mut self) -> C5S_W<CSELRrs> {
        C5S_W::new(self, 16)
    }
    ///Bits 20:23 - DMA channel 6 selection
    #[inline(always)]
    pub fn c6s(&mut self) -> C6S_W<CSELRrs> {
        C6S_W::new(self, 20)
    }
    ///Bits 24:27 - DMA channel 7 selection
    #[inline(always)]
    pub fn c7s(&mut self) -> C7S_W<CSELRrs> {
        C7S_W::new(self, 24)
    }
}
/**channel selection register

You can [`read`](crate::Reg::read) this register and get [`cselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32WB55.html#DMA2:CSELR)*/
pub struct CSELRrs;
impl crate::RegisterSpec for CSELRrs {
    type Ux = u32;
}
///`read()` method returns [`cselr::R`](R) reader structure
impl crate::Readable for CSELRrs {}
///`write(|w| ..)` method takes [`cselr::W`](W) writer structure
impl crate::Writable for CSELRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CSELR to value 0
impl crate::Resettable for CSELRrs {
    const RESET_VALUE: u32 = 0;
}
