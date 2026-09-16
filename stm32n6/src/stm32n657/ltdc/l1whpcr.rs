///Register `L1WHPCR` reader
pub type R = crate::R<L1WHPCRrs>;
///Register `L1WHPCR` writer
pub type W = crate::W<L1WHPCRrs>;
///Field `WHSTPOS` reader - window horizontal start position
pub type WHSTPOS_R = crate::FieldReader<u16>;
///Field `WHSTPOS` writer - window horizontal start position
pub type WHSTPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `WHSPPOS` reader - window horizontal stop position
pub type WHSPPOS_R = crate::FieldReader<u16>;
///Field `WHSPPOS` writer - window horizontal stop position
pub type WHSPPOS_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - window horizontal start position
    #[inline(always)]
    pub fn whstpos(&self) -> WHSTPOS_R {
        WHSTPOS_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:31 - window horizontal stop position
    #[inline(always)]
    pub fn whsppos(&self) -> WHSPPOS_R {
        WHSPPOS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1WHPCR")
            .field("whstpos", &self.whstpos())
            .field("whsppos", &self.whsppos())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - window horizontal start position
    #[inline(always)]
    pub fn whstpos(&mut self) -> WHSTPOS_W<'_, L1WHPCRrs> {
        WHSTPOS_W::new(self, 0)
    }
    ///Bits 16:31 - window horizontal stop position
    #[inline(always)]
    pub fn whsppos(&mut self) -> WHSPPOS_W<'_, L1WHPCRrs> {
        WHSPPOS_W::new(self, 16)
    }
}
/**LTDC layerx window horizontal position configuration register

You can [`read`](crate::Reg::read) this register and get [`l1whpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1whpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#LTDC:L1WHPCR)*/
pub struct L1WHPCRrs;
impl crate::RegisterSpec for L1WHPCRrs {
    type Ux = u32;
}
///`read()` method returns [`l1whpcr::R`](R) reader structure
impl crate::Readable for L1WHPCRrs {}
///`write(|w| ..)` method takes [`l1whpcr::W`](W) writer structure
impl crate::Writable for L1WHPCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1WHPCR to value 0
impl crate::Resettable for L1WHPCRrs {}
