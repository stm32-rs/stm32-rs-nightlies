///Register `L1CR` reader
pub type R = crate::R<L1CRrs>;
///Register `L1CR` writer
pub type W = crate::W<L1CRrs>;
///Field `LEN` reader - LEN
pub type LEN_R = crate::BitReader;
///Field `LEN` writer - LEN
pub type LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COLKEN` reader - COLKEN
pub type COLKEN_R = crate::BitReader;
///Field `COLKEN` writer - COLKEN
pub type COLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLUTEN` reader - CLUTEN
pub type CLUTEN_R = crate::BitReader;
///Field `CLUTEN` writer - CLUTEN
pub type CLUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - LEN
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - COLKEN
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - CLUTEN
    #[inline(always)]
    pub fn cluten(&self) -> CLUTEN_R {
        CLUTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L1CR")
            .field("len", &self.len())
            .field("colken", &self.colken())
            .field("cluten", &self.cluten())
            .finish()
    }
}
impl W {
    ///Bit 0 - LEN
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<'_, L1CRrs> {
        LEN_W::new(self, 0)
    }
    ///Bit 1 - COLKEN
    #[inline(always)]
    pub fn colken(&mut self) -> COLKEN_W<'_, L1CRrs> {
        COLKEN_W::new(self, 1)
    }
    ///Bit 4 - CLUTEN
    #[inline(always)]
    pub fn cluten(&mut self) -> CLUTEN_W<'_, L1CRrs> {
        CLUTEN_W::new(self, 4)
    }
}
/**LTDC layer 1 control register

You can [`read`](crate::Reg::read) this register and get [`l1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#LTDC:L1CR)*/
pub struct L1CRrs;
impl crate::RegisterSpec for L1CRrs {
    type Ux = u32;
}
///`read()` method returns [`l1cr::R`](R) reader structure
impl crate::Readable for L1CRrs {}
///`write(|w| ..)` method takes [`l1cr::W`](W) writer structure
impl crate::Writable for L1CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets L1CR to value 0
impl crate::Resettable for L1CRrs {}
