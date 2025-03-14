///Register `L1CR` reader
pub type R = crate::R<L1CRrs>;
///Register `L1CR` writer
pub type W = crate::W<L1CRrs>;
///Field `LEN` reader - layer enable This bit is set and cleared by software.
pub type LEN_R = crate::BitReader;
///Field `LEN` writer - layer enable This bit is set and cleared by software.
pub type LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `COLKEN` reader - color keying enable This bit is set and cleared by software.
pub type COLKEN_R = crate::BitReader;
///Field `COLKEN` writer - color keying enable This bit is set and cleared by software.
pub type COLKEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CLUTEN` reader - color look-up table enable This bit is set and cleared by software. The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to table (CLUT)
pub type CLUTEN_R = crate::BitReader;
///Field `CLUTEN` writer - color look-up table enable This bit is set and cleared by software. The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to table (CLUT)
pub type CLUTEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - layer enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn len(&self) -> LEN_R {
        LEN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - color keying enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn colken(&self) -> COLKEN_R {
        COLKEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - color look-up table enable This bit is set and cleared by software. The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to table (CLUT)
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
    ///Bit 0 - layer enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn len(&mut self) -> LEN_W<L1CRrs> {
        LEN_W::new(self, 0)
    }
    ///Bit 1 - color keying enable This bit is set and cleared by software.
    #[inline(always)]
    pub fn colken(&mut self) -> COLKEN_W<L1CRrs> {
        COLKEN_W::new(self, 1)
    }
    ///Bit 4 - color look-up table enable This bit is set and cleared by software. The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to table (CLUT)
    #[inline(always)]
    pub fn cluten(&mut self) -> CLUTEN_W<L1CRrs> {
        CLUTEN_W::new(self, 4)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`l1cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:L1CR)*/
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
