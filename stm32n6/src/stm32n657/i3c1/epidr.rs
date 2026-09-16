///Register `EPIDR` reader
pub type R = crate::R<EPIDRrs>;
///Register `EPIDR` writer
pub type W = crate::W<EPIDRrs>;
///Field `MIPIID` reader - 4-bit MIPI Instance ID
pub type MIPIID_R = crate::FieldReader;
///Field `MIPIID` writer - 4-bit MIPI Instance ID
pub type MIPIID_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `IDTSEL` reader - provisioned ID type selector
pub type IDTSEL_R = crate::BitReader;
///Field `MIPIMID` reader - 15-bit MIPI manufacturer ID
pub type MIPIMID_R = crate::FieldReader<u16>;
impl R {
    ///Bits 12:15 - 4-bit MIPI Instance ID
    #[inline(always)]
    pub fn mipiid(&self) -> MIPIID_R {
        MIPIID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    ///Bit 16 - provisioned ID type selector
    #[inline(always)]
    pub fn idtsel(&self) -> IDTSEL_R {
        IDTSEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bits 17:31 - 15-bit MIPI manufacturer ID
    #[inline(always)]
    pub fn mipimid(&self) -> MIPIMID_R {
        MIPIMID_R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EPIDR")
            .field("mipiid", &self.mipiid())
            .field("idtsel", &self.idtsel())
            .field("mipimid", &self.mipimid())
            .finish()
    }
}
impl W {
    ///Bits 12:15 - 4-bit MIPI Instance ID
    #[inline(always)]
    pub fn mipiid(&mut self) -> MIPIID_W<'_, EPIDRrs> {
        MIPIID_W::new(self, 12)
    }
}
/**I3C extended provisioned ID register

You can [`read`](crate::Reg::read) this register and get [`epidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`epidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N657.html#I3C1:EPIDR)*/
pub struct EPIDRrs;
impl crate::RegisterSpec for EPIDRrs {
    type Ux = u32;
}
///`read()` method returns [`epidr::R`](R) reader structure
impl crate::Readable for EPIDRrs {}
///`write(|w| ..)` method takes [`epidr::W`](W) writer structure
impl crate::Writable for EPIDRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets EPIDR to value 0x0208_0000
impl crate::Resettable for EPIDRrs {
    const RESET_VALUE: u32 = 0x0208_0000;
}
