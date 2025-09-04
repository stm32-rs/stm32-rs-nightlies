///Register `GHCR` reader
pub type R = crate::R<GHCRrs>;
///Register `GHCR` writer
pub type W = crate::W<GHCRrs>;
///Field `DT` reader - Type
pub type DT_R = crate::FieldReader;
///Field `DT` writer - Type
pub type DT_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
///Field `VCID` reader - Channel
pub type VCID_R = crate::FieldReader;
///Field `VCID` writer - Channel
pub type VCID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `WCLSB` reader - WordCount LSB
pub type WCLSB_R = crate::FieldReader;
///Field `WCLSB` writer - WordCount LSB
pub type WCLSB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WCMSB` reader - WordCount MSB
pub type WCMSB_R = crate::FieldReader;
///Field `WCMSB` writer - WordCount MSB
pub type WCMSB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:5 - Type
    #[inline(always)]
    pub fn dt(&self) -> DT_R {
        DT_R::new((self.bits & 0x3f) as u8)
    }
    ///Bits 6:7 - Channel
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:15 - WordCount LSB
    #[inline(always)]
    pub fn wclsb(&self) -> WCLSB_R {
        WCLSB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - WordCount MSB
    #[inline(always)]
    pub fn wcmsb(&self) -> WCMSB_R {
        WCMSB_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GHCR")
            .field("dt", &self.dt())
            .field("vcid", &self.vcid())
            .field("wclsb", &self.wclsb())
            .field("wcmsb", &self.wcmsb())
            .finish()
    }
}
impl W {
    ///Bits 0:5 - Type
    #[inline(always)]
    pub fn dt(&mut self) -> DT_W<GHCRrs> {
        DT_W::new(self, 0)
    }
    ///Bits 6:7 - Channel
    #[inline(always)]
    pub fn vcid(&mut self) -> VCID_W<GHCRrs> {
        VCID_W::new(self, 6)
    }
    ///Bits 8:15 - WordCount LSB
    #[inline(always)]
    pub fn wclsb(&mut self) -> WCLSB_W<GHCRrs> {
        WCLSB_W::new(self, 8)
    }
    ///Bits 16:23 - WordCount MSB
    #[inline(always)]
    pub fn wcmsb(&mut self) -> WCMSB_W<GHCRrs> {
        WCMSB_W::new(self, 16)
    }
}
/**DSI Host generic header configuration register

You can [`read`](crate::Reg::read) this register and get [`ghcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ghcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H757_CM4.html#DSIHOST:GHCR)*/
pub struct GHCRrs;
impl crate::RegisterSpec for GHCRrs {
    type Ux = u32;
}
///`read()` method returns [`ghcr::R`](R) reader structure
impl crate::Readable for GHCRrs {}
///`write(|w| ..)` method takes [`ghcr::W`](W) writer structure
impl crate::Writable for GHCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GHCR to value 0
impl crate::Resettable for GHCRrs {}
