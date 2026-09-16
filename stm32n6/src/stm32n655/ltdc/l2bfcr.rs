///Register `L2BFCR` reader
pub type R = crate::R<L2BFCRrs>;
///Register `L2BFCR` writer
pub type W = crate::W<L2BFCRrs>;
///Field `BF2` reader - blending factor 2
pub type BF2_R = crate::FieldReader;
///Field `BF2` writer - blending factor 2
pub type BF2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BF1` reader - blending factor 1
pub type BF1_R = crate::FieldReader;
///Field `BF1` writer - blending factor 1
pub type BF1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `BOR` reader - blending order
pub type BOR_R = crate::BitReader;
///Field `BOR` writer - blending order
pub type BOR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:2 - blending factor 2
    #[inline(always)]
    pub fn bf2(&self) -> BF2_R {
        BF2_R::new((self.bits & 7) as u8)
    }
    ///Bits 8:10 - blending factor 1
    #[inline(always)]
    pub fn bf1(&self) -> BF1_R {
        BF1_R::new(((self.bits >> 8) & 7) as u8)
    }
    ///Bit 16 - blending order
    #[inline(always)]
    pub fn bor(&self) -> BOR_R {
        BOR_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2BFCR")
            .field("bf2", &self.bf2())
            .field("bf1", &self.bf1())
            .field("bor", &self.bor())
            .finish()
    }
}
impl W {
    ///Bits 0:2 - blending factor 2
    #[inline(always)]
    pub fn bf2(&mut self) -> BF2_W<'_, L2BFCRrs> {
        BF2_W::new(self, 0)
    }
    ///Bits 8:10 - blending factor 1
    #[inline(always)]
    pub fn bf1(&mut self) -> BF1_W<'_, L2BFCRrs> {
        BF1_W::new(self, 8)
    }
    ///Bit 16 - blending order
    #[inline(always)]
    pub fn bor(&mut self) -> BOR_W<'_, L2BFCRrs> {
        BOR_W::new(self, 16)
    }
}
/**LTDC layerx blending factors configuration register

You can [`read`](crate::Reg::read) this register and get [`l2bfcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2bfcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N655.html#LTDC:L2BFCR)*/
pub struct L2BFCRrs;
impl crate::RegisterSpec for L2BFCRrs {
    type Ux = u32;
}
///`read()` method returns [`l2bfcr::R`](R) reader structure
impl crate::Readable for L2BFCRrs {}
///`write(|w| ..)` method takes [`l2bfcr::W`](W) writer structure
impl crate::Writable for L2BFCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2BFCR to value 0x0001_0607
impl crate::Resettable for L2BFCRrs {
    const RESET_VALUE: u32 = 0x0001_0607;
}
