///Register `L2CR` reader
pub type R = crate::R<L2CRrs>;
///Register `L2CR` writer
pub type W = crate::W<L2CRrs>;
///Field `LEN` reader - Layer Enable
pub type LEN_R = crate::BitReader;
///Field `LEN` writer - Layer Enable
pub type LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COLKEN` reader - Color Keying Enable
pub type COLKEN_R = crate::BitReader;
///Field `COLKEN` writer - Color Keying Enable
pub type COLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLUTEN` reader - Color Look-Up Table Enable
pub type CLUTEN_R = crate::BitReader;
///Field `CLUTEN` writer - Color Look-Up Table Enable
pub type CLUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Layer Enable
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Color Keying Enable
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - Color Look-Up Table Enable
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2CR")
            .field("len", &self.len())
            .field("colken", &self.colken())
            .field("cluten", &self.cluten())
            .finish()
    }
}
impl W {
    ///Bit 0 - Layer Enable
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<L2CRrs> {
        LEN_W::new(self, 0)
    }
    ///Bit 1 - Color Keying Enable
    #[inline(always)]
    pub fn colken(&mut self) -> COLKEN_W<L2CRrs> {
        COLKEN_W::new(self, 1)
    }
    ///Bit 4 - Color Look-Up Table Enable
    #[inline(always)]
    pub fn cluten(&mut self) -> CLUTEN_W<L2CRrs> {
        CLUTEN_W::new(self, 4)
    }
}
/**LTDC Layer Control Register

You can [`read`](crate::Reg::read) this register and get [`l2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32L4R5.html#LTDC:L2CR)*/
pub struct L2CRrs;
impl crate::RegisterSpec for L2CRrs {
    type Ux = u32;
}
///`read()` method returns [`l2cr::R`](R) reader structure
impl crate::Readable for L2CRrs {}
///`write(|w| ..)` method takes [`l2cr::W`](W) writer structure
impl crate::Writable for L2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L2CR to value 0
impl crate::Resettable for L2CRrs {}
