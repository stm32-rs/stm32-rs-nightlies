///Register `GFC` reader
pub type R = crate::R<GFCrs>;
///Register `GFC` writer
pub type W = crate::W<GFCrs>;
///Field `RRFE` reader - Reject Remote Frames Extended
pub type RRFE_R = crate::BitReader;
///Field `RRFE` writer - Reject Remote Frames Extended
pub type RRFE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RRFS` reader - Reject Remote Frames Standard
pub type RRFS_R = crate::BitReader;
///Field `RRFS` writer - Reject Remote Frames Standard
pub type RRFS_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANFE` reader - Accept Non-matching Frames Extended
pub type ANFE_R = crate::FieldReader;
///Field `ANFE` writer - Accept Non-matching Frames Extended
pub type ANFE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `ANFS` reader - Accept Non-matching Frames Standard
pub type ANFS_R = crate::FieldReader;
///Field `ANFS` writer - Accept Non-matching Frames Standard
pub type ANFS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Reject Remote Frames Extended
    #[inline(always)]
    pub fn rrfe(&self) -> RRFE_R {
        RRFE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Reject Remote Frames Standard
    #[inline(always)]
    pub fn rrfs(&self) -> RRFS_R {
        RRFS_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Accept Non-matching Frames Extended
    #[inline(always)]
    pub fn anfe(&self) -> ANFE_R {
        ANFE_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:5 - Accept Non-matching Frames Standard
    #[inline(always)]
    pub fn anfs(&self) -> ANFS_R {
        ANFS_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GFC")
            .field("rrfe", &self.rrfe())
            .field("rrfs", &self.rrfs())
            .field("anfe", &self.anfe())
            .field("anfs", &self.anfs())
            .finish()
    }
}
impl W {
    ///Bit 0 - Reject Remote Frames Extended
    #[inline(always)]
    pub fn rrfe(&mut self) -> RRFE_W<'_, GFCrs> {
        RRFE_W::new(self, 0)
    }
    ///Bit 1 - Reject Remote Frames Standard
    #[inline(always)]
    pub fn rrfs(&mut self) -> RRFS_W<'_, GFCrs> {
        RRFS_W::new(self, 1)
    }
    ///Bits 2:3 - Accept Non-matching Frames Extended
    #[inline(always)]
    pub fn anfe(&mut self) -> ANFE_W<'_, GFCrs> {
        ANFE_W::new(self, 2)
    }
    ///Bits 4:5 - Accept Non-matching Frames Standard
    #[inline(always)]
    pub fn anfs(&mut self) -> ANFS_W<'_, GFCrs> {
        ANFS_W::new(self, 4)
    }
}
/**FDCAN Global Filter Configuration Register

You can [`read`](crate::Reg::read) this register and get [`gfc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gfc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H743.html#FDCAN1:GFC)*/
pub struct GFCrs;
impl crate::RegisterSpec for GFCrs {
    type Ux = u32;
}
///`read()` method returns [`gfc::R`](R) reader structure
impl crate::Readable for GFCrs {}
///`write(|w| ..)` method takes [`gfc::W`](W) writer structure
impl crate::Writable for GFCrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets GFC to value 0
impl crate::Resettable for GFCrs {}
