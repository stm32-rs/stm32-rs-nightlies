///Register `L2CR` reader
pub type R = crate::R<L2CRrs>;
///Register `L2CR` writer
pub type W = crate::W<L2CRrs>;
///Field `LEN` reader - layer enable
pub type LEN_R = crate::BitReader;
///Field `LEN` writer - layer enable
pub type LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CKEN` reader - color keying enable
pub type CKEN_R = crate::BitReader;
///Field `CKEN` writer - color keying enable
pub type CKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLUTEN` reader - color look-up table enable
pub type CLUTEN_R = crate::BitReader;
///Field `CLUTEN` writer - color look-up table enable
pub type CLUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HMEN` reader - horizontal mirroring enable
pub type HMEN_R = crate::BitReader;
///Field `HMEN` writer - horizontal mirroring enable
pub type HMEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DCBEN` reader - default color blending enable
pub type DCBEN_R = crate::BitReader;
///Field `DCBEN` writer - default color blending enable
pub type DCBEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - layer enable
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - color keying enable
    #[inline(always)]
    pub fn cken(&self) -> CKEN_R {
        CKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - color look-up table enable
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 8 - horizontal mirroring enable
    #[inline(always)]
    pub fn hmen(&self) -> HMEN_R {
        HMEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - default color blending enable
    #[inline(always)]
    pub fn dcben(&self) -> DCBEN_R {
        DCBEN_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2CR")
            .field("len", &self.len())
            .field("cken", &self.cken())
            .field("cluten", &self.cluten())
            .field("hmen", &self.hmen())
            .field("dcben", &self.dcben())
            .finish()
    }
}
impl W {
    ///Bit 0 - layer enable
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<L2CRrs> {
        LEN_W::new(self, 0)
    }
    ///Bit 1 - color keying enable
    #[inline(always)]
    pub fn cken(&mut self) -> CKEN_W<L2CRrs> {
        CKEN_W::new(self, 1)
    }
    ///Bit 4 - color look-up table enable
    #[inline(always)]
    pub fn cluten(&mut self) -> CLUTEN_W<L2CRrs> {
        CLUTEN_W::new(self, 4)
    }
    ///Bit 8 - horizontal mirroring enable
    #[inline(always)]
    pub fn hmen(&mut self) -> HMEN_W<L2CRrs> {
        HMEN_W::new(self, 8)
    }
    ///Bit 9 - default color blending enable
    #[inline(always)]
    pub fn dcben(&mut self) -> DCBEN_W<L2CRrs> {
        DCBEN_W::new(self, 9)
    }
}
/**LTDC layerx control register

You can [`read`](crate::Reg::read) this register and get [`l2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#LTDC:L2CR)*/
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
