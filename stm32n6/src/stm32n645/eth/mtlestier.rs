///Register `MTLESTIER` reader
pub type R = crate::R<MTLESTIERrs>;
///Register `MTLESTIER` writer
pub type W = crate::W<MTLESTIERrs>;
///Field `IECC` reader - Interrupt Enable for Switch List
pub type IECC_R = crate::BitReader;
///Field `IECC` writer - Interrupt Enable for Switch List
pub type IECC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IEBE` reader - Interrupt Enable for BTR Error
pub type IEBE_R = crate::BitReader;
///Field `IEBE` writer - Interrupt Enable for BTR Error
pub type IEBE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IEHF` reader - Interrupt Enable for HLBF
pub type IEHF_R = crate::BitReader;
///Field `IEHF` writer - Interrupt Enable for HLBF
pub type IEHF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IEHS` reader - Interrupt Enable for HLBS
pub type IEHS_R = crate::BitReader;
///Field `IEHS` writer - Interrupt Enable for HLBS
pub type IEHS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CGCE` reader - Interrupt Enable for CGCE
pub type CGCE_R = crate::BitReader;
///Field `CGCE` writer - Interrupt Enable for CGCE
pub type CGCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Interrupt Enable for Switch List
    #[inline(always)]
    pub fn iecc(&self) -> IECC_R {
        IECC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Interrupt Enable for BTR Error
    #[inline(always)]
    pub fn iebe(&self) -> IEBE_R {
        IEBE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Interrupt Enable for HLBF
    #[inline(always)]
    pub fn iehf(&self) -> IEHF_R {
        IEHF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Interrupt Enable for HLBS
    #[inline(always)]
    pub fn iehs(&self) -> IEHS_R {
        IEHS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Interrupt Enable for CGCE
    #[inline(always)]
    pub fn cgce(&self) -> CGCE_R {
        CGCE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLESTIER")
            .field("iecc", &self.iecc())
            .field("iebe", &self.iebe())
            .field("iehf", &self.iehf())
            .field("iehs", &self.iehs())
            .field("cgce", &self.cgce())
            .finish()
    }
}
impl W {
    ///Bit 0 - Interrupt Enable for Switch List
    #[inline(always)]
    pub fn iecc(&mut self) -> IECC_W<'_, MTLESTIERrs> {
        IECC_W::new(self, 0)
    }
    ///Bit 1 - Interrupt Enable for BTR Error
    #[inline(always)]
    pub fn iebe(&mut self) -> IEBE_W<'_, MTLESTIERrs> {
        IEBE_W::new(self, 1)
    }
    ///Bit 2 - Interrupt Enable for HLBF
    #[inline(always)]
    pub fn iehf(&mut self) -> IEHF_W<'_, MTLESTIERrs> {
        IEHF_W::new(self, 2)
    }
    ///Bit 3 - Interrupt Enable for HLBS
    #[inline(always)]
    pub fn iehs(&mut self) -> IEHS_W<'_, MTLESTIERrs> {
        IEHS_W::new(self, 3)
    }
    ///Bit 4 - Interrupt Enable for CGCE
    #[inline(always)]
    pub fn cgce(&mut self) -> CGCE_W<'_, MTLESTIERrs> {
        CGCE_W::new(self, 4)
    }
}
/**EST Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`mtlestier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N645.html#ETH:MTLESTIER)*/
pub struct MTLESTIERrs;
impl crate::RegisterSpec for MTLESTIERrs {
    type Ux = u32;
}
///`read()` method returns [`mtlestier::R`](R) reader structure
impl crate::Readable for MTLESTIERrs {}
///`write(|w| ..)` method takes [`mtlestier::W`](W) writer structure
impl crate::Writable for MTLESTIERrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLESTIER to value 0
impl crate::Resettable for MTLESTIERrs {}
