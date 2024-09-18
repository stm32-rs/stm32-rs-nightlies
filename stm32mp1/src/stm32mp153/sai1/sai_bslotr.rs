///Register `SAI_BSLOTR` reader
pub type R = crate::R<SAI_BSLOTRrs>;
///Register `SAI_BSLOTR` writer
pub type W = crate::W<SAI_BSLOTRrs>;
///Field `FBOFF` reader - FBOFF
pub type FBOFF_R = crate::FieldReader;
///Field `FBOFF` writer - FBOFF
pub type FBOFF_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
///Field `SLOTSZ` reader - SLOTSZ
pub type SLOTSZ_R = crate::FieldReader;
///Field `SLOTSZ` writer - SLOTSZ
pub type SLOTSZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `NBSLOT` reader - NBSLOT
pub type NBSLOT_R = crate::FieldReader;
///Field `NBSLOT` writer - NBSLOT
pub type NBSLOT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `SLOTEN` reader - SLOTEN
pub type SLOTEN_R = crate::FieldReader<u16>;
///Field `SLOTEN` writer - SLOTEN
pub type SLOTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:4 - FBOFF
    #[inline(always)]
    pub fn fboff(&self) -> FBOFF_R {
        FBOFF_R::new((self.bits & 0x1f) as u8)
    }
    ///Bits 6:7 - SLOTSZ
    #[inline(always)]
    pub fn slotsz(&self) -> SLOTSZ_R {
        SLOTSZ_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bits 8:11 - NBSLOT
    #[inline(always)]
    pub fn nbslot(&self) -> NBSLOT_R {
        NBSLOT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    ///Bits 16:31 - SLOTEN
    #[inline(always)]
    pub fn sloten(&self) -> SLOTEN_R {
        SLOTEN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAI_BSLOTR")
            .field("fboff", &self.fboff())
            .field("slotsz", &self.slotsz())
            .field("nbslot", &self.nbslot())
            .field("sloten", &self.sloten())
            .finish()
    }
}
impl W {
    ///Bits 0:4 - FBOFF
    #[inline(always)]
    #[must_use]
    pub fn fboff(&mut self) -> FBOFF_W<SAI_BSLOTRrs> {
        FBOFF_W::new(self, 0)
    }
    ///Bits 6:7 - SLOTSZ
    #[inline(always)]
    #[must_use]
    pub fn slotsz(&mut self) -> SLOTSZ_W<SAI_BSLOTRrs> {
        SLOTSZ_W::new(self, 6)
    }
    ///Bits 8:11 - NBSLOT
    #[inline(always)]
    #[must_use]
    pub fn nbslot(&mut self) -> NBSLOT_W<SAI_BSLOTRrs> {
        NBSLOT_W::new(self, 8)
    }
    ///Bits 16:31 - SLOTEN
    #[inline(always)]
    #[must_use]
    pub fn sloten(&mut self) -> SLOTEN_W<SAI_BSLOTRrs> {
        SLOTEN_W::new(self, 16)
    }
}
/**This register has no meaning in and SPDIF audio protocol

You can [`read`](crate::Reg::read) this register and get [`sai_bslotr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_bslotr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#SAI1:SAI_BSLOTR)*/
pub struct SAI_BSLOTRrs;
impl crate::RegisterSpec for SAI_BSLOTRrs {
    type Ux = u32;
}
///`read()` method returns [`sai_bslotr::R`](R) reader structure
impl crate::Readable for SAI_BSLOTRrs {}
///`write(|w| ..)` method takes [`sai_bslotr::W`](W) writer structure
impl crate::Writable for SAI_BSLOTRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SAI_BSLOTR to value 0
impl crate::Resettable for SAI_BSLOTRrs {
    const RESET_VALUE: u32 = 0;
}
