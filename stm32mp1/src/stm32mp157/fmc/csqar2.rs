///Register `CSQAR2` reader
pub type R = crate::R<CSQAR2rs>;
///Register `CSQAR2` writer
pub type W = crate::W<CSQAR2rs>;
///Field `ADDC5` reader - ADDC5
pub type ADDC5_R = crate::FieldReader;
///Field `ADDC5` writer - ADDC5
pub type ADDC5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `NANDCEN0` reader - NANDCEN0
pub type NANDCEN0_R = crate::BitReader;
///Field `NANDCEN0` writer - NANDCEN0
pub type NANDCEN0_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `NANDCEN1` reader - NANDCEN1
pub type NANDCEN1_R = crate::BitReader;
///Field `NANDCEN1` writer - NANDCEN1
pub type NANDCEN1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SAO` reader - SAO
pub type SAO_R = crate::FieldReader<u16>;
///Field `SAO` writer - SAO
pub type SAO_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:7 - ADDC5
    #[inline(always)]
    pub fn addc5(&self) -> ADDC5_R {
        ADDC5_R::new((self.bits & 0xff) as u8)
    }
    ///Bit 10 - NANDCEN0
    #[inline(always)]
    pub fn nandcen0(&self) -> NANDCEN0_R {
        NANDCEN0_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - NANDCEN1
    #[inline(always)]
    pub fn nandcen1(&self) -> NANDCEN1_R {
        NANDCEN1_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 16:31 - SAO
    #[inline(always)]
    pub fn sao(&self) -> SAO_R {
        SAO_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CSQAR2")
            .field("addc5", &self.addc5())
            .field("nandcen0", &self.nandcen0())
            .field("nandcen1", &self.nandcen1())
            .field("sao", &self.sao())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - ADDC5
    #[inline(always)]
    pub fn addc5(&mut self) -> ADDC5_W<'_, CSQAR2rs> {
        ADDC5_W::new(self, 0)
    }
    ///Bit 10 - NANDCEN0
    #[inline(always)]
    pub fn nandcen0(&mut self) -> NANDCEN0_W<'_, CSQAR2rs> {
        NANDCEN0_W::new(self, 10)
    }
    ///Bit 11 - NANDCEN1
    #[inline(always)]
    pub fn nandcen1(&mut self) -> NANDCEN1_W<'_, CSQAR2rs> {
        NANDCEN1_W::new(self, 11)
    }
    ///Bits 16:31 - SAO
    #[inline(always)]
    pub fn sao(&mut self) -> SAO_W<'_, CSQAR2rs> {
        SAO_W::new(self, 16)
    }
}
/**This register is used to program the fifth address cycle and the address offset in spare area. It also selects the chip enable.

You can [`read`](crate::Reg::read) this register and get [`csqar2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csqar2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32MP157.html#FMC:CSQAR2)*/
pub struct CSQAR2rs;
impl crate::RegisterSpec for CSQAR2rs {
    type Ux = u32;
}
///`read()` method returns [`csqar2::R`](R) reader structure
impl crate::Readable for CSQAR2rs {}
///`write(|w| ..)` method takes [`csqar2::W`](W) writer structure
impl crate::Writable for CSQAR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CSQAR2 to value 0x0002_0000
impl crate::Resettable for CSQAR2rs {
    const RESET_VALUE: u32 = 0x0002_0000;
}
