///Register `SADDR` reader
pub type R = crate::R<SADDRrs>;
///Register `SADDR` writer
pub type W = crate::W<SADDRrs>;
///Field `BADDSTART` reader - Region address start This bitfield defines the absolute start address of the region x on 4 kBytes boundary (inclusive). BREN cannot be set if BADDRSTART > BADDREND. When MCE determines the region, the first 12 bits (LSB) and the last 4 bits (MSB) in this register are ignored, and when this register is accessed in read the 4 MSB bits and the 12 LSB bits return zeros (reference value in MCE).
pub type BADDSTART_R = crate::FieldReader<u32>;
///Field `BADDSTART` writer - Region address start This bitfield defines the absolute start address of the region x on 4 kBytes boundary (inclusive). BREN cannot be set if BADDRSTART > BADDREND. When MCE determines the region, the first 12 bits (LSB) and the last 4 bits (MSB) in this register are ignored, and when this register is accessed in read the 4 MSB bits and the 12 LSB bits return zeros (reference value in MCE).
pub type BADDSTART_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 12:31 - Region address start This bitfield defines the absolute start address of the region x on 4 kBytes boundary (inclusive). BREN cannot be set if BADDRSTART > BADDREND. When MCE determines the region, the first 12 bits (LSB) and the last 4 bits (MSB) in this register are ignored, and when this register is accessed in read the 4 MSB bits and the 12 LSB bits return zeros (reference value in MCE).
    #[inline(always)]
    pub fn baddstart(&self) -> BADDSTART_R {
        BADDSTART_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SADDR")
            .field("baddstart", &self.baddstart())
            .finish()
    }
}
impl W {
    ///Bits 12:31 - Region address start This bitfield defines the absolute start address of the region x on 4 kBytes boundary (inclusive). BREN cannot be set if BADDRSTART > BADDREND. When MCE determines the region, the first 12 bits (LSB) and the last 4 bits (MSB) in this register are ignored, and when this register is accessed in read the 4 MSB bits and the 12 LSB bits return zeros (reference value in MCE).
    #[inline(always)]
    pub fn baddstart(&mut self) -> BADDSTART_W<'_, SADDRrs> {
        BADDSTART_W::new(self, 12)
    }
}
/**Region start address register

You can [`read`](crate::Reg::read) this register and get [`saddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`saddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SADDRrs;
impl crate::RegisterSpec for SADDRrs {
    type Ux = u32;
}
///`read()` method returns [`saddr::R`](R) reader structure
impl crate::Readable for SADDRrs {}
///`write(|w| ..)` method takes [`saddr::W`](W) writer structure
impl crate::Writable for SADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SADDR to value 0
impl crate::Resettable for SADDRrs {}
