///Register `SMCR` reader
pub type R = crate::R<SMCRrs>;
///Register `SMCR` writer
pub type W = crate::W<SMCRrs>;
///Field `BKPRWDPROT` reader - BKPRWDPROT
pub type BKPRWDPROT_R = crate::FieldReader;
///Field `BKPRWDPROT` writer - BKPRWDPROT
pub type BKPRWDPROT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `BKPWDPROT` reader - BKPWDPROT
pub type BKPWDPROT_R = crate::FieldReader;
///Field `BKPWDPROT` writer - BKPWDPROT
pub type BKPWDPROT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `TAMPDPROT` reader - TAMPDPROT
pub type TAMPDPROT_R = crate::BitReader;
///Field `TAMPDPROT` writer - TAMPDPROT
pub type TAMPDPROT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:7 - BKPRWDPROT
    #[inline(always)]
    pub fn bkprwdprot(&self) -> BKPRWDPROT_R {
        BKPRWDPROT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 16:23 - BKPWDPROT
    #[inline(always)]
    pub fn bkpwdprot(&self) -> BKPWDPROT_R {
        BKPWDPROT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    ///Bit 31 - TAMPDPROT
    #[inline(always)]
    pub fn tampdprot(&self) -> TAMPDPROT_R {
        TAMPDPROT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMCR")
            .field("bkprwdprot", &self.bkprwdprot())
            .field("bkpwdprot", &self.bkpwdprot())
            .field("tampdprot", &self.tampdprot())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - BKPRWDPROT
    #[inline(always)]
    pub fn bkprwdprot(&mut self) -> BKPRWDPROT_W<'_, SMCRrs> {
        BKPRWDPROT_W::new(self, 0)
    }
    ///Bits 16:23 - BKPWDPROT
    #[inline(always)]
    pub fn bkpwdprot(&mut self) -> BKPWDPROT_W<'_, SMCRrs> {
        BKPWDPROT_W::new(self, 16)
    }
    ///Bit 31 - TAMPDPROT
    #[inline(always)]
    pub fn tampdprot(&mut self) -> TAMPDPROT_W<'_, SMCRrs> {
        TAMPDPROT_W::new(self, 31)
    }
}
/**This register can be written only when the APB access is secure.

You can [`read`](crate::Reg::read) this register and get [`smcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP153.html#TAMP:SMCR)*/
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
///`read()` method returns [`smcr::R`](R) reader structure
impl crate::Readable for SMCRrs {}
///`write(|w| ..)` method takes [`smcr::W`](W) writer structure
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets SMCR to value 0x8000_0000
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0x8000_0000;
}
