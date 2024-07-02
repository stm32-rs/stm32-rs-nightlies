///Register `LTDC_L2CR` reader
pub type R = crate::R<LTDC_L2CRrs>;
///Register `LTDC_L2CR` writer
pub type W = crate::W<LTDC_L2CRrs>;
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
        f.debug_struct("LTDC_L2CR")
            .field("len", &self.len())
            .field("colken", &self.colken())
            .field("cluten", &self.cluten())
            .finish()
    }
}
impl W {
    ///Bit 0 - layer enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn len(&mut self) -> LEN_W<LTDC_L2CRrs> {
        LEN_W::new(self, 0)
    }
    ///Bit 1 - color keying enable This bit is set and cleared by software.
    #[inline(always)]
    #[must_use]
    pub fn colken(&mut self) -> COLKEN_W<LTDC_L2CRrs> {
        COLKEN_W::new(self, 1)
    }
    ///Bit 4 - color look-up table enable This bit is set and cleared by software. The CLUT is only meaningful for L8, AL44 and AL88 pixel format. Refer to table (CLUT)
    #[inline(always)]
    #[must_use]
    pub fn cluten(&mut self) -> CLUTEN_W<LTDC_L2CRrs> {
        CLUTEN_W::new(self, 4)
    }
}
/**

You can [`read`](crate::Reg::read) this register and get [`ltdc_l2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltdc_l2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U5A9.html#LTDC:LTDC_L2CR)*/
pub struct LTDC_L2CRrs;
impl crate::RegisterSpec for LTDC_L2CRrs {
    type Ux = u32;
}
///`read()` method returns [`ltdc_l2cr::R`](R) reader structure
impl crate::Readable for LTDC_L2CRrs {}
///`write(|w| ..)` method takes [`ltdc_l2cr::W`](W) writer structure
impl crate::Writable for LTDC_L2CRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LTDC_L2CR to value 0
impl crate::Resettable for LTDC_L2CRrs {
    const RESET_VALUE: u32 = 0;
}
