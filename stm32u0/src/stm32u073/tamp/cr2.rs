///Register `CR2` reader
pub type R = crate::R<CR2rs>;
///Register `CR2` writer
pub type W = crate::W<CR2rs>;
///Field `TAMP1POM` reader - Tamper 1 potential mode
pub type TAMP1POM_R = crate::BitReader;
///Field `TAMP1POM` writer - Tamper 1 potential mode
pub type TAMP1POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2POM` reader - Tamper 2 potential mode
pub type TAMP2POM_R = crate::BitReader;
///Field `TAMP2POM` writer - Tamper 2 potential mode
pub type TAMP2POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3POM` reader - Tamper 3 potential mode
pub type TAMP3POM_R = crate::BitReader;
///Field `TAMP3POM` writer - Tamper 3 potential mode
pub type TAMP3POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP4POM` reader - Tamper 4 potential mode
pub type TAMP4POM_R = crate::BitReader;
///Field `TAMP4POM` writer - Tamper 4 potential mode
pub type TAMP4POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP5POM` reader - Tamper 5 potential mode
pub type TAMP5POM_R = crate::BitReader;
///Field `TAMP5POM` writer - Tamper 5 potential mode
pub type TAMP5POM_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1MSK` reader - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
pub type TAMP1MSK_R = crate::BitReader;
///Field `TAMP1MSK` writer - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
pub type TAMP1MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2MSK` reader - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
pub type TAMP2MSK_R = crate::BitReader;
///Field `TAMP2MSK` writer - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
pub type TAMP2MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3MSK` reader - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.
pub type TAMP3MSK_R = crate::BitReader;
///Field `TAMP3MSK` writer - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.
pub type TAMP3MSK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKBLOCK` reader - Backup registers and device secrets<sup>(1)</sup> access blocked
pub type BKBLOCK_R = crate::BitReader;
///Field `BKBLOCK` writer - Backup registers and device secrets<sup>(1)</sup> access blocked
pub type BKBLOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `BKERASE` writer - Backup registers and device secrets<sup>(1)</sup> erase Writing 1 to this bit reset the backup registers and device secrets<sup>(1)</sup>. Writing 0 has no effect. This bit is always read as 0.
pub type BKERASE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP1TRG` reader - Active level for tamper 1 input If TAMPFLT1=100 tamper 1 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 1 input falling edge triggers a tamper detection event.
pub type TAMP1TRG_R = crate::BitReader;
///Field `TAMP1TRG` writer - Active level for tamper 1 input If TAMPFLT1=100 tamper 1 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 1 input falling edge triggers a tamper detection event.
pub type TAMP1TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP2TRG` reader - Active level for tamper 2 input If TAMPFLT = 00 tamper 2 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 2 input falling edge triggers a tamper detection event.
pub type TAMP2TRG_R = crate::BitReader;
///Field `TAMP2TRG` writer - Active level for tamper 2 input If TAMPFLT = 00 tamper 2 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 2 input falling edge triggers a tamper detection event.
pub type TAMP2TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP3TRG` reader - Active level for tamper 3 input If TAMPFLT1=100 tamper 3 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 3 input falling edge triggers a tamper detection event.
pub type TAMP3TRG_R = crate::BitReader;
///Field `TAMP3TRG` writer - Active level for tamper 3 input If TAMPFLT1=100 tamper 3 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 3 input falling edge triggers a tamper detection event.
pub type TAMP3TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP4TRG` reader - Active level for tamper 4 input (active mode disabled) If TAMPFLT1=100 tamper 4 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 4 input falling edge triggers a tamper detection event.
pub type TAMP4TRG_R = crate::BitReader;
///Field `TAMP4TRG` writer - Active level for tamper 4 input (active mode disabled) If TAMPFLT1=100 tamper 4 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 4 input falling edge triggers a tamper detection event.
pub type TAMP4TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TAMP5TRG` reader - Active level for tamper 5 input (active mode disabled) If TAMPFLT1=100 tamper 5 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 5 input falling edge triggers a tamper detection event.
pub type TAMP5TRG_R = crate::BitReader;
///Field `TAMP5TRG` writer - Active level for tamper 5 input (active mode disabled) If TAMPFLT1=100 tamper 5 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 5 input falling edge triggers a tamper detection event.
pub type TAMP5TRG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Tamper 1 potential mode
    #[inline(always)]
    pub fn tamp1pom(&self) -> TAMP1POM_R {
        TAMP1POM_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Tamper 2 potential mode
    #[inline(always)]
    pub fn tamp2pom(&self) -> TAMP2POM_R {
        TAMP2POM_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Tamper 3 potential mode
    #[inline(always)]
    pub fn tamp3pom(&self) -> TAMP3POM_R {
        TAMP3POM_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Tamper 4 potential mode
    #[inline(always)]
    pub fn tamp4pom(&self) -> TAMP4POM_R {
        TAMP4POM_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Tamper 5 potential mode
    #[inline(always)]
    pub fn tamp5pom(&self) -> TAMP5POM_R {
        TAMP5POM_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
    #[inline(always)]
    pub fn tamp1msk(&self) -> TAMP1MSK_R {
        TAMP1MSK_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
    #[inline(always)]
    pub fn tamp2msk(&self) -> TAMP2MSK_R {
        TAMP2MSK_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.
    #[inline(always)]
    pub fn tamp3msk(&self) -> TAMP3MSK_R {
        TAMP3MSK_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 22 - Backup registers and device secrets<sup>(1)</sup> access blocked
    #[inline(always)]
    pub fn bkblock(&self) -> BKBLOCK_R {
        BKBLOCK_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 24 - Active level for tamper 1 input If TAMPFLT1=100 tamper 1 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 1 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp1trg(&self) -> TAMP1TRG_R {
        TAMP1TRG_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - Active level for tamper 2 input If TAMPFLT = 00 tamper 2 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 2 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp2trg(&self) -> TAMP2TRG_R {
        TAMP2TRG_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Active level for tamper 3 input If TAMPFLT1=100 tamper 3 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 3 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp3trg(&self) -> TAMP3TRG_R {
        TAMP3TRG_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - Active level for tamper 4 input (active mode disabled) If TAMPFLT1=100 tamper 4 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 4 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp4trg(&self) -> TAMP4TRG_R {
        TAMP4TRG_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - Active level for tamper 5 input (active mode disabled) If TAMPFLT1=100 tamper 5 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 5 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp5trg(&self) -> TAMP5TRG_R {
        TAMP5TRG_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CR2")
            .field("tamp1pom", &self.tamp1pom())
            .field("tamp2pom", &self.tamp2pom())
            .field("tamp3pom", &self.tamp3pom())
            .field("tamp4pom", &self.tamp4pom())
            .field("tamp5pom", &self.tamp5pom())
            .field("tamp1msk", &self.tamp1msk())
            .field("tamp2msk", &self.tamp2msk())
            .field("tamp3msk", &self.tamp3msk())
            .field("bkblock", &self.bkblock())
            .field("tamp1trg", &self.tamp1trg())
            .field("tamp2trg", &self.tamp2trg())
            .field("tamp3trg", &self.tamp3trg())
            .field("tamp4trg", &self.tamp4trg())
            .field("tamp5trg", &self.tamp5trg())
            .finish()
    }
}
impl W {
    ///Bit 0 - Tamper 1 potential mode
    #[inline(always)]
    pub fn tamp1pom(&mut self) -> TAMP1POM_W<'_, CR2rs> {
        TAMP1POM_W::new(self, 0)
    }
    ///Bit 1 - Tamper 2 potential mode
    #[inline(always)]
    pub fn tamp2pom(&mut self) -> TAMP2POM_W<'_, CR2rs> {
        TAMP2POM_W::new(self, 1)
    }
    ///Bit 2 - Tamper 3 potential mode
    #[inline(always)]
    pub fn tamp3pom(&mut self) -> TAMP3POM_W<'_, CR2rs> {
        TAMP3POM_W::new(self, 2)
    }
    ///Bit 3 - Tamper 4 potential mode
    #[inline(always)]
    pub fn tamp4pom(&mut self) -> TAMP4POM_W<'_, CR2rs> {
        TAMP4POM_W::new(self, 3)
    }
    ///Bit 4 - Tamper 5 potential mode
    #[inline(always)]
    pub fn tamp5pom(&mut self) -> TAMP5POM_W<'_, CR2rs> {
        TAMP5POM_W::new(self, 4)
    }
    ///Bit 16 - Tamper 1 mask The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
    #[inline(always)]
    pub fn tamp1msk(&mut self) -> TAMP1MSK_W<'_, CR2rs> {
        TAMP1MSK_W::new(self, 16)
    }
    ///Bit 17 - Tamper 2 mask The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
    #[inline(always)]
    pub fn tamp2msk(&mut self) -> TAMP2MSK_W<'_, CR2rs> {
        TAMP2MSK_W::new(self, 17)
    }
    ///Bit 18 - Tamper 3 mask The tamper 3 interrupt must not be enabled when TAMP3MSK is set.
    #[inline(always)]
    pub fn tamp3msk(&mut self) -> TAMP3MSK_W<'_, CR2rs> {
        TAMP3MSK_W::new(self, 18)
    }
    ///Bit 22 - Backup registers and device secrets<sup>(1)</sup> access blocked
    #[inline(always)]
    pub fn bkblock(&mut self) -> BKBLOCK_W<'_, CR2rs> {
        BKBLOCK_W::new(self, 22)
    }
    ///Bit 23 - Backup registers and device secrets<sup>(1)</sup> erase Writing 1 to this bit reset the backup registers and device secrets<sup>(1)</sup>. Writing 0 has no effect. This bit is always read as 0.
    #[inline(always)]
    pub fn bkerase(&mut self) -> BKERASE_W<'_, CR2rs> {
        BKERASE_W::new(self, 23)
    }
    ///Bit 24 - Active level for tamper 1 input If TAMPFLT1=100 tamper 1 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 1 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> TAMP1TRG_W<'_, CR2rs> {
        TAMP1TRG_W::new(self, 24)
    }
    ///Bit 25 - Active level for tamper 2 input If TAMPFLT = 00 tamper 2 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 2 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> TAMP2TRG_W<'_, CR2rs> {
        TAMP2TRG_W::new(self, 25)
    }
    ///Bit 26 - Active level for tamper 3 input If TAMPFLT1=100 tamper 3 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 3 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> TAMP3TRG_W<'_, CR2rs> {
        TAMP3TRG_W::new(self, 26)
    }
    ///Bit 27 - Active level for tamper 4 input (active mode disabled) If TAMPFLT1=100 tamper 4 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 4 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp4trg(&mut self) -> TAMP4TRG_W<'_, CR2rs> {
        TAMP4TRG_W::new(self, 27)
    }
    ///Bit 28 - Active level for tamper 5 input (active mode disabled) If TAMPFLT1=100 tamper 5 input rising edge triggers a tamper detection event. If TAMPFLT1=100 tamper 5 input falling edge triggers a tamper detection event.
    #[inline(always)]
    pub fn tamp5trg(&mut self) -> TAMP5TRG_W<'_, CR2rs> {
        TAMP5TRG_W::new(self, 28)
    }
}
/**TAMP control register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

See register [structure](https://stm32-rs.github.io/stm32-rs/STM32U073.html#TAMP:CR2)*/
pub struct CR2rs;
impl crate::RegisterSpec for CR2rs {
    type Ux = u32;
}
///`read()` method returns [`cr2::R`](R) reader structure
impl crate::Readable for CR2rs {}
///`write(|w| ..)` method takes [`cr2::W`](W) writer structure
impl crate::Writable for CR2rs {
    type Safety = crate::Unsafe;
}
///`reset()` method sets CR2 to value 0
impl crate::Resettable for CR2rs {}
