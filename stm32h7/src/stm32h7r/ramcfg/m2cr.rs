///Register `M2CR` reader
pub type R = crate::R<M2CRrs>;
///Register `M2CR` writer
pub type W = crate::W<M2CRrs>;
///Field `ECCSEIE` reader - ECC single error interrupt enable When ECCSEIE bit is set to 1, monitor x generates an interrupt when an ECC single error occurs during a read operation from RAM.
pub type ECCSEIE_R = crate::BitReader;
///Field `ECCSEIE` writer - ECC single error interrupt enable When ECCSEIE bit is set to 1, monitor x generates an interrupt when an ECC single error occurs during a read operation from RAM.
pub type ECCSEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCDEIE` reader - ECC double error interrupt enable When ECCDEIE bit is set to 1, monitor x generates an interrupt when an ECC double detection error occurs during a read operation from RAM.
pub type ECCDEIE_R = crate::BitReader;
///Field `ECCDEIE` writer - ECC double error interrupt enable When ECCDEIE bit is set to 1, monitor x generates an interrupt when an ECC double detection error occurs during a read operation from RAM.
pub type ECCDEIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCDEBWIE` reader - ECC double error on byte write (BW) interrupt enable When ECCDEBWIE bit is set to 1, monitor x generates an interrupt when an ECC double detection error occurs during a byte write operation to RAM.
pub type ECCDEBWIE_R = crate::BitReader;
///Field `ECCDEBWIE` writer - ECC double error on byte write (BW) interrupt enable When ECCDEBWIE bit is set to 1, monitor x generates an interrupt when an ECC double detection error occurs during a byte write operation to RAM.
pub type ECCDEBWIE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCELEN` reader - ECC error latching enable When ECCELEN bit is set to 1, if an ECC error occurs (both for single error correction or double detection) during a read operation, the context (address, data and ECC code) that generated the error are latched to their respective registers.
pub type ECCELEN_R = crate::BitReader;
///Field `ECCELEN` writer - ECC error latching enable When ECCELEN bit is set to 1, if an ECC error occurs (both for single error correction or double detection) during a read operation, the context (address, data and ECC code) that generated the error are latched to their respective registers.
pub type ECCELEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCSECEN` reader - ECC single error counter enable When ECCSECEN bit is set to 1, the occurrence counter is incremented when an ECC single error occurs during a read operation from RAM.
pub type ECCSECEN_R = crate::BitReader;
///Field `ECCSECEN` writer - ECC single error counter enable When ECCSECEN bit is set to 1, the occurrence counter is incremented when an ECC single error occurs during a read operation from RAM.
pub type ECCSECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCDECEN` reader - ECC double error counter enable When ECCDECEN bit is set to 1, the occurrence counter is incremented when an ECC double detection error occurs during a read operation from RAM.
pub type ECCDECEN_R = crate::BitReader;
///Field `ECCDECEN` writer - ECC double error counter enable When ECCDECEN bit is set to 1, the occurrence counter is incremented when an ECC double detection error occurs during a read operation from RAM.
pub type ECCDECEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCDEBWCEN` reader - ECC double error on byte write (BW) counter enable When ECCDEBWCEN bit is set to 1, the occurrence counter is incremented when an ECC double detection error occurs during a byte write operation to RAM.
pub type ECCDEBWCEN_R = crate::BitReader;
///Field `ECCDEBWCEN` writer - ECC double error on byte write (BW) counter enable When ECCDEBWCEN bit is set to 1, the occurrence counter is incremented when an ECC double detection error occurs during a byte write operation to RAM.
pub type ECCDEBWCEN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ECCTEA` reader - ECC Test ECC access
pub type ECCTEA_R = crate::FieldReader;
///Field `ECCTEA` writer - ECC Test ECC access
pub type ECCTEA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 2 - ECC single error interrupt enable When ECCSEIE bit is set to 1, monitor x generates an interrupt when an ECC single error occurs during a read operation from RAM.
    #[inline(always)]
    pub fn eccseie(&self) -> ECCSEIE_R {
        ECCSEIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - ECC double error interrupt enable When ECCDEIE bit is set to 1, monitor x generates an interrupt when an ECC double detection error occurs during a read operation from RAM.
    #[inline(always)]
    pub fn eccdeie(&self) -> ECCDEIE_R {
        ECCDEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - ECC double error on byte write (BW) interrupt enable When ECCDEBWIE bit is set to 1, monitor x generates an interrupt when an ECC double detection error occurs during a byte write operation to RAM.
    #[inline(always)]
    pub fn eccdebwie(&self) -> ECCDEBWIE_R {
        ECCDEBWIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - ECC error latching enable When ECCELEN bit is set to 1, if an ECC error occurs (both for single error correction or double detection) during a read operation, the context (address, data and ECC code) that generated the error are latched to their respective registers.
    #[inline(always)]
    pub fn eccelen(&self) -> ECCELEN_R {
        ECCELEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - ECC single error counter enable When ECCSECEN bit is set to 1, the occurrence counter is incremented when an ECC single error occurs during a read operation from RAM.
    #[inline(always)]
    pub fn eccsecen(&self) -> ECCSECEN_R {
        ECCSECEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - ECC double error counter enable When ECCDECEN bit is set to 1, the occurrence counter is incremented when an ECC double detection error occurs during a read operation from RAM.
    #[inline(always)]
    pub fn eccdecen(&self) -> ECCDECEN_R {
        ECCDECEN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - ECC double error on byte write (BW) counter enable When ECCDEBWCEN bit is set to 1, the occurrence counter is incremented when an ECC double detection error occurs during a byte write operation to RAM.
    #[inline(always)]
    pub fn eccdebwcen(&self) -> ECCDEBWCEN_R {
        ECCDEBWCEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 16:17 - ECC Test ECC access
    #[inline(always)]
    pub fn ecctea(&self) -> ECCTEA_R {
        ECCTEA_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M2CR")
            .field("eccseie", &self.eccseie())
            .field("eccdeie", &self.eccdeie())
            .field("eccdebwie", &self.eccdebwie())
            .field("eccelen", &self.eccelen())
            .field("eccsecen", &self.eccsecen())
            .field("eccdecen", &self.eccdecen())
            .field("eccdebwcen", &self.eccdebwcen())
            .field("ecctea", &self.ecctea())
            .finish()
    }
}
impl W {
    ///Bit 2 - ECC single error interrupt enable When ECCSEIE bit is set to 1, monitor x generates an interrupt when an ECC single error occurs during a read operation from RAM.
    #[inline(always)]
    pub fn eccseie(&mut self) -> ECCSEIE_W<'_, M2CRrs> {
        ECCSEIE_W::new(self, 2)
    }
    ///Bit 3 - ECC double error interrupt enable When ECCDEIE bit is set to 1, monitor x generates an interrupt when an ECC double detection error occurs during a read operation from RAM.
    #[inline(always)]
    pub fn eccdeie(&mut self) -> ECCDEIE_W<'_, M2CRrs> {
        ECCDEIE_W::new(self, 3)
    }
    ///Bit 4 - ECC double error on byte write (BW) interrupt enable When ECCDEBWIE bit is set to 1, monitor x generates an interrupt when an ECC double detection error occurs during a byte write operation to RAM.
    #[inline(always)]
    pub fn eccdebwie(&mut self) -> ECCDEBWIE_W<'_, M2CRrs> {
        ECCDEBWIE_W::new(self, 4)
    }
    ///Bit 5 - ECC error latching enable When ECCELEN bit is set to 1, if an ECC error occurs (both for single error correction or double detection) during a read operation, the context (address, data and ECC code) that generated the error are latched to their respective registers.
    #[inline(always)]
    pub fn eccelen(&mut self) -> ECCELEN_W<'_, M2CRrs> {
        ECCELEN_W::new(self, 5)
    }
    ///Bit 6 - ECC single error counter enable When ECCSECEN bit is set to 1, the occurrence counter is incremented when an ECC single error occurs during a read operation from RAM.
    #[inline(always)]
    pub fn eccsecen(&mut self) -> ECCSECEN_W<'_, M2CRrs> {
        ECCSECEN_W::new(self, 6)
    }
    ///Bit 7 - ECC double error counter enable When ECCDECEN bit is set to 1, the occurrence counter is incremented when an ECC double detection error occurs during a read operation from RAM.
    #[inline(always)]
    pub fn eccdecen(&mut self) -> ECCDECEN_W<'_, M2CRrs> {
        ECCDECEN_W::new(self, 7)
    }
    ///Bit 8 - ECC double error on byte write (BW) counter enable When ECCDEBWCEN bit is set to 1, the occurrence counter is incremented when an ECC double detection error occurs during a byte write operation to RAM.
    #[inline(always)]
    pub fn eccdebwcen(&mut self) -> ECCDEBWCEN_W<'_, M2CRrs> {
        ECCDEBWCEN_W::new(self, 8)
    }
    ///Bits 16:17 - ECC Test ECC access
    #[inline(always)]
    pub fn ecctea(&mut self) -> ECCTEA_W<'_, M2CRrs> {
        ECCTEA_W::new(self, 16)
    }
}
/**RAMECC monitor 2 configuration register

You can [`read`](crate::Reg::read) this register and get [`m2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32H7R.html#RAMCFG:M2CR)*/
pub struct M2CRrs;
impl crate::RegisterSpec for M2CRrs {
    type Ux = u32;
}
///`read()` method returns [`m2cr::R`](R) reader structure
impl crate::Readable for M2CRrs {}
///`write(|w| ..)` method takes [`m2cr::W`](W) writer structure
impl crate::Writable for M2CRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets M2CR to value 0
impl crate::Resettable for M2CRrs {}
