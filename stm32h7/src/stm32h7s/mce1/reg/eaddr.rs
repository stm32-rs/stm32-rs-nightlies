///Register `EADDR` reader
pub type R = crate::R<EADDRrs>;
///Register `EADDR` writer
pub type W = crate::W<EADDRrs>;
///Field `BADDEND` reader - Region address end This bitfield defines the absolute end address of the region x on 4 kBytes boundary (inclusive). BREN cannot be set if BADDRSTART > BADDREND. When MCE determines the region, the first 12 bits (LSB) and the last 4 bits (MSB) in this register are ignored, and when this register is accessed in read the 4 MSB bits return zeros and the 12 LSB bits return ones (reference value in MCE).
pub type BADDEND_R = crate::FieldReader<u32>;
///Field `BADDEND` writer - Region address end This bitfield defines the absolute end address of the region x on 4 kBytes boundary (inclusive). BREN cannot be set if BADDRSTART > BADDREND. When MCE determines the region, the first 12 bits (LSB) and the last 4 bits (MSB) in this register are ignored, and when this register is accessed in read the 4 MSB bits return zeros and the 12 LSB bits return ones (reference value in MCE).
pub type BADDEND_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 12:31 - Region address end This bitfield defines the absolute end address of the region x on 4 kBytes boundary (inclusive). BREN cannot be set if BADDRSTART > BADDREND. When MCE determines the region, the first 12 bits (LSB) and the last 4 bits (MSB) in this register are ignored, and when this register is accessed in read the 4 MSB bits return zeros and the 12 LSB bits return ones (reference value in MCE).
    #[inline(always)]
    pub fn baddend(&self) -> BADDEND_R {
        BADDEND_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EADDR")
            .field("baddend", &self.baddend())
            .finish()
    }
}
impl W {
    ///Bits 12:31 - Region address end This bitfield defines the absolute end address of the region x on 4 kBytes boundary (inclusive). BREN cannot be set if BADDRSTART > BADDREND. When MCE determines the region, the first 12 bits (LSB) and the last 4 bits (MSB) in this register are ignored, and when this register is accessed in read the 4 MSB bits return zeros and the 12 LSB bits return ones (reference value in MCE).
    #[inline(always)]
    pub fn baddend(&mut self) -> BADDEND_W<'_, EADDRrs> {
        BADDEND_W::new(self, 12)
    }
}
/**Region end address register

You can [`read`](crate::Reg::read) this register and get [`eaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EADDRrs;
impl crate::RegisterSpec for EADDRrs {
    type Ux = u32;
}
///`read()` method returns [`eaddr::R`](R) reader structure
impl crate::Readable for EADDRrs {}
///`write(|w| ..)` method takes [`eaddr::W`](W) writer structure
impl crate::Writable for EADDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EADDR to value 0x0fff
impl crate::Resettable for EADDRrs {
    const RESET_VALUE: u32 = 0x0fff;
}
