///Register `MTLESTSR` reader
pub type R = crate::R<MTLESTSRrs>;
///Register `MTLESTSR` writer
pub type W = crate::W<MTLESTSRrs>;
///Field `SWLC` reader - Switch to S/W owned list Complete
pub type SWLC_R = crate::BitReader;
///Field `SWLC` writer - Switch to S/W owned list Complete
pub type SWLC_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BTRE` reader - BTR Error
pub type BTRE_R = crate::BitReader;
///Field `BTRE` writer - BTR Error
pub type BTRE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HLBF` reader - Head-Of-Line Blocking due to Frame Size
pub type HLBF_R = crate::BitReader;
///Field `HLBS` reader - Head-Of-Line Blocking due to Scheduling
pub type HLBS_R = crate::BitReader;
///Field `CGCE` reader - Constant Gate Control Error
pub type CGCE_R = crate::BitReader;
///Field `CGCE` writer - Constant Gate Control Error
pub type CGCE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SWOL` reader - S/W owned list
pub type SWOL_R = crate::BitReader;
///Field `BTRL` reader - BTR Error Loop Count
pub type BTRL_R = crate::FieldReader;
///Field `CGSN` reader - Current GCL slot number
pub type CGSN_R = crate::FieldReader;
impl R {
    ///Bit 0 - Switch to S/W owned list Complete
    #[inline(always)]
    pub fn swlc(&self) -> SWLC_R {
        SWLC_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - BTR Error
    #[inline(always)]
    pub fn btre(&self) -> BTRE_R {
        BTRE_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Head-Of-Line Blocking due to Frame Size
    #[inline(always)]
    pub fn hlbf(&self) -> HLBF_R {
        HLBF_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Head-Of-Line Blocking due to Scheduling
    #[inline(always)]
    pub fn hlbs(&self) -> HLBS_R {
        HLBS_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Constant Gate Control Error
    #[inline(always)]
    pub fn cgce(&self) -> CGCE_R {
        CGCE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - S/W owned list
    #[inline(always)]
    pub fn swol(&self) -> SWOL_R {
        SWOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bits 8:15 - BTR Error Loop Count
    #[inline(always)]
    pub fn btrl(&self) -> BTRL_R {
        BTRL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:19 - Current GCL slot number
    #[inline(always)]
    pub fn cgsn(&self) -> CGSN_R {
        CGSN_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTLESTSR")
            .field("swlc", &self.swlc())
            .field("btre", &self.btre())
            .field("hlbf", &self.hlbf())
            .field("hlbs", &self.hlbs())
            .field("cgce", &self.cgce())
            .field("swol", &self.swol())
            .field("btrl", &self.btrl())
            .field("cgsn", &self.cgsn())
            .finish()
    }
}
impl W {
    ///Bit 0 - Switch to S/W owned list Complete
    #[inline(always)]
    pub fn swlc(&mut self) -> SWLC_W<'_, MTLESTSRrs> {
        SWLC_W::new(self, 0)
    }
    ///Bit 1 - BTR Error
    #[inline(always)]
    pub fn btre(&mut self) -> BTRE_W<'_, MTLESTSRrs> {
        BTRE_W::new(self, 1)
    }
    ///Bit 4 - Constant Gate Control Error
    #[inline(always)]
    pub fn cgce(&mut self) -> CGCE_W<'_, MTLESTSRrs> {
        CGCE_W::new(self, 4)
    }
}
/**EST Status Register

You can [`read`](crate::Reg::read) this register and get [`mtlestsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlestsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32N647.html#ETH:MTLESTSR)*/
pub struct MTLESTSRrs;
impl crate::RegisterSpec for MTLESTSRrs {
    type Ux = u32;
}
///`read()` method returns [`mtlestsr::R`](R) reader structure
impl crate::Readable for MTLESTSRrs {}
///`write(|w| ..)` method takes [`mtlestsr::W`](W) writer structure
impl crate::Writable for MTLESTSRrs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets MTLESTSR to value 0
impl crate::Resettable for MTLESTSRrs {}
